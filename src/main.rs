use gnuplot::{Axes2D, Color, Figure, PointSize, PointSymbol};
use ndarray::Array2;
use std::f64::consts;

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

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn random_point(sz: usize) -> Self {
        let sz = sz as f64;
        let x = (fastrand::f64() - 0.5) * 2. * sz;
        let y = (fastrand::f64() - 0.5) * 2. * sz;

        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        let (x1, y1) = self.coordinates();
        let (x2, y2) = other.coordinates();
        let dx = x1 - x2;
        let dy = y1 - y2;
        (dx * dx + dy * dy).sqrt()
    }

    fn norm(&self) -> f64 {
        let (x, y) = self.coordinates();
        (x * x + y * y).sqrt()
    }

    fn coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn translate(&self, dist: f64, angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let dx = c * dist;
        let dy = s * dist;
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn is_inside(&self, map_size: usize) -> bool {
        let map_size = map_size as f64;
        self.x.abs() < map_size && self.y.abs() < map_size
    }
}

fn get_initial_point(size: usize, cap: &mut CommulativeAcceptProbability) -> Point {
    loop {
        let point = Point::random_point(size);
        let prob = cap.get_probabiliy(&point);
        if prob > fastrand::f64() {
            cap.add_point(point);
            return point;
        }
    }
}

fn get_initial_direction(initial: &Point) -> f64 {
    let (x, y) = initial.coordinates();
    let angle = fastrand::f64() * consts::FRAC_PI_2;
    angle
        + match (x.is_sign_positive(), y.is_sign_positive()) {
            (false, false) => 0.,
            (true, false) => consts::FRAC_PI_2,
            (true, true) => consts::PI,
            (false, true) => 3. * consts::FRAC_PI_2,
        }
}

fn draw_ancors(map_size: usize, cap: &mut CommulativeAcceptProbability) -> Vec<Point> {
    let mut initial = get_initial_point(map_size, cap);
    let mut direction = get_initial_direction(&initial);
    let mut output = vec![];
    while initial.is_inside(map_size) {
        output.push(initial);
        let noise = (fastrand::f64() - 0.5) * consts::FRAC_PI_8;
        direction += noise;
        initial = initial.translate(250., direction);
    }

    output
}

fn draw_points(points: &[Point], axes: &mut Axes2D, symbol: Option<char>) {
    let x = points.iter().map(|p| p.x);
    let y = points.iter().map(|p| p.y);

    axes.points(x, y, &[PointSize(2.), PointSymbol(symbol.unwrap_or('S'))]);
}

fn draw_line(p1: &Point, p2: &Point, axes: &mut Axes2D) {
    let (x1, y1) = p1.coordinates();
    let (x2, y2) = p2.coordinates();
    axes.lines([x1, x2], [y1, y2], &[Color("black")]);
}

fn draw_lines(points: &[Point], adj_mat: &Array2<f64>, axes: &mut Axes2D) {
    for ((i, j), adj) in adj_mat.indexed_iter() {
        if *adj < f64::MAX {
            draw_line(&points[i], &points[j], axes);
        }
    }
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

    fn build_adjacent_matrix(&self, dist: f64) -> Array2<f64> {
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
        self.all_cross_line_iter(|line_a, line_b| {
            add_close_point_arcs(line_a, line_b, dist, dist, &mut output)
        });
        output
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
            let index = if dist < d1 {
                Some((ba + i, bb + j))
            } else if dist < d2 {
                Some((ba + i, bb + j))
            } else {
                None
            };
            if let Some((i, j)) = index {
                mat[(i, j)] = dist;
                mat[(j, i)] = dist;
            }
        }
    }
}

fn main() {
    let mut fig = Figure::new();
    let axes = fig.axes2d();
    let map_size = 1000;
    let mut collect = CollectPoints::default();
    let mut cap = CommulativeAcceptProbability::new(map_size as f64, 100., 450.);
    for _ in 0..4 {
        let points = draw_ancors(map_size, &mut cap);
        collect.collect(points);
    }

    for line in collect.line_iter() {
        draw_points(line, axes, None);
    }

    let adj_mat = collect.build_adjacent_matrix(150.);

    draw_lines(&collect.points, &adj_mat, axes);
    fig.show().unwrap();
}
