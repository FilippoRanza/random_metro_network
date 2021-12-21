use ndarray::Array2;
use serde::Deserialize;
use std::f64::consts;
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod check_connection;
mod plotter;
mod point;
mod rand_utils;
use point::Point;

pub trait Localizable {
    fn coordinates(&self) -> (f64, f64);
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}

struct CommulativeAcceptProbability {
    initials: Vec<Point>,
    map_size: f64,
    scale: f64,
    min_dist: f64,
}

impl CommulativeAcceptProbability {
    fn new(map_size: f64, scale: f64, min_dist: f64) -> Self {
        Self {
            map_size,
            scale,
            min_dist,
            initials: vec![],
        }
    }

    fn get_probabiliy(&self, p: &Point) -> f64 {
        let begin = self.accept_prob(p.norm(), self.map_size);
        self.initials
            .iter()
            .map(|p2| p.distance(p2))
            .fold(begin, |acc, curr| {
                acc + self.accept_prob(curr, self.min_dist)
            })
    }

    fn add_point(&mut self, p: Point) {
        self.initials.push(p);
    }

    fn accept_prob(&self, dist: f64, min_dist: f64) -> f64 {
        let diff = (dist - min_dist) / self.scale;
        let diff = diff * diff;
        let diff = -diff;
        diff.exp()
    }
}

fn get_initial_point(size: f64, cap: &mut CommulativeAcceptProbability) -> Point {
    loop {
        let point = Point::random_point(size);
        let prob = cap.get_probabiliy(&point);
        if prob > rand_utils::rand_f64() {
            cap.add_point(point);
            return point;
        }
    }
}

fn get_initial_direction(initial: &Point) -> f64 {
    let (x, y) = initial.coordinates();
    let angle = rand_utils::random_below(consts::FRAC_PI_2);
    angle
        + match (x.is_sign_positive(), y.is_sign_positive()) {
            (false, false) => 0.,
            (true, false) => consts::FRAC_PI_2,
            (true, true) => consts::PI,
            (false, true) => 3. * consts::FRAC_PI_2,
        }
}

fn make_random_stations(
    map_size: f64,
    avg_dist: f64,
    var_dist: f64,
    cap: &mut CommulativeAcceptProbability,
) -> Vec<Point> {
    let mut initial = get_initial_point(map_size, cap);
    let mut direction = get_initial_direction(&initial);
    let mut output = vec![];
    while initial.is_inside(map_size) {
        output.push(initial);
        let noise = rand_utils::random_in(consts::FRAC_PI_8);
        direction += noise;
        let dist = rand_utils::rand_normal(avg_dist, var_dist);
        initial = initial.translate(dist, direction);
    }

    output
}

#[derive(Default, Debug)]
struct CollectPoints {
    points: Vec<Point>,
    lines: Vec<(usize, usize)>,
}
impl CollectPoints {
    fn collect(&mut self, mut points: Vec<Point>) {
        let begin = self.points.len();
        let end = points.len() + begin;
        self.lines.push((begin, end));
        self.points.append(&mut points);
    }

    fn line_iter(&self) -> impl Iterator<Item = &'_ [Point]> {
        self.index_line_iter().map(|(_, o)| o)
    }

    fn index_line_iter(&self) -> impl Iterator<Item = ((usize, usize), &'_ [Point])> {
        self.lines
            .iter()
            .copied()
            .map(|(a, b)| ((a, b), &self.points[a..b]))
    }

    fn build_adjacent_matrix(&mut self, dist: f64) -> Array2<f64> {
        let output = self.build_line_adj();
        self.build_collapse_adj(output, dist)
    }

    fn build_collapse_adj(&mut self, mut mat: Array2<f64>, dist: f64) -> Array2<f64> {
        let near = self.collapse_stations(dist);
        for (s1, s2) in near.collapse_station {
            mat[(s1, s2)] = 0.;
            mat[(s2, s1)] = 0.;
        }
        mat
    }

    fn build_line_adj(&self) -> Array2<f64> {
        let mut output = Array2::from_elem((self.points.len(), self.points.len()), f64::MAX);
        for (b, e) in &self.lines {
            let mut prev: Option<(usize, &Point)> = None;
            let line = &self.points[*b..*e];
            for (i, p) in line.iter().enumerate() {
                let i = i + b;
                if let Some((prev_i, prev_p)) = prev {
                    let d = prev_p.distance(p);
                    output[(prev_i, i)] = d;
                    output[(i, prev_i)] = d;
                }
                prev = Some((i, p));
            }
        }
        output
    }

    fn collapse_stations(&mut self, dist: f64) -> NearStationFind {
        let mut find_near = NearStationFind::new();
        self.all_cross_line_iter(|a, b| find_near.find_near(a, b, dist));
        for (s1, s2) in &find_near.collapse_station {
            let new_point = self.points[*s1].find_middle(&self.points[*s2]);
            self.points[*s1] = new_point;
            self.points[*s2] = new_point;
        }
        find_near
    }

    fn all_cross_line_iter<F>(&self, mut f: F)
    where
        F: FnMut(((usize, usize), &'_ [Point]), ((usize, usize), &'_ [Point])),
    {
        for (i, line_a) in self.index_line_iter().enumerate() {
            for line_b in self.index_line_iter().skip(i + 1) {
                f(line_a, line_b);
            }
        }
    }
}

fn add_close_point_arcs(
    line_a: ((usize, usize), &[Point]),
    line_b: ((usize, usize), &[Point]),
    d1: f64,
    d2: f64,
    mat: &mut Array2<f64>,
) {
    let ((ba, _), line_a) = line_a;
    let ((bb, _), line_b) = line_b;

    for (i, a) in line_a.iter().enumerate() {
        for (j, b) in line_b.iter().enumerate() {
            let dist = a.distance(b);
            if dist < d1 || dist < d2 {
                let (i, j) = (ba + i, bb + j);
                mat[(i, j)] = dist;
                mat[(j, i)] = dist;
            }
        }
    }
}

#[derive(Default)]
struct NearStationFind {
    collapse_station: Vec<(usize, usize)>,
}
impl NearStationFind {
    fn new() -> Self {
        Self::default()
    }

    fn find_near(
        &mut self,
        line_a: ((usize, usize), &[Point]),
        line_b: ((usize, usize), &[Point]),
        d: f64,
    ) {
        let ((ba, _), line_a) = line_a;
        let ((bb, _), line_b) = line_b;

        for (i, a) in line_a.iter().enumerate() {
            for (j, b) in line_b.iter().enumerate() {
                let dist = a.distance(b);
                if dist < d {
                    self.collapse_station.push((ba + i, bb + j));
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct Config {
    map_size: f64,
    scale: f64,
    min_dist: f64,
    line_count: usize,
    station_avg_distance: f64,
    station_var_distance: f64,
    collapse_station_distance: f64,
}

#[derive(StructOpt)]
struct Arguments {
    config: PathBuf,
}

fn load_config(file_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let config = serde_yaml::from_reader(file)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::from_args();
    let config = load_config(&args.config)?;
    let mut fig = plotter::PlotFigure::new();
    let mut plot = fig.make_plotter();
    let map_size = config.map_size;
    let mut collect = CollectPoints::default();
    let mut cap = CommulativeAcceptProbability::new(map_size, config.scale, config.min_dist);
    for _ in 0..config.line_count {
        let points = make_random_stations(
            map_size,
            config.station_avg_distance,
            config.station_var_distance,
            &mut cap,
        );
        collect.collect(points);
    }

    let adj_mat = collect.build_adjacent_matrix(config.collapse_station_distance);

    for line in collect.line_iter() {
        plot.draw_points(line, None);
    }

    if !check_connection::is_connected(&adj_mat) {
        println!("Network is not connected");
    }

    plot.draw_lines(&collect.points, &adj_mat);
    fig.show();

    Ok(())
}
