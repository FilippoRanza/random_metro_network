use flo_curves::bezier;
use flo_curves::Coord2;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use std::fs::File;
use std::path::PathBuf;

type Curve = bezier::Curve<Coord2>;

mod bezier_point_factory;
mod build_graph;
mod float_table;
mod intersections;
mod make_curves;
mod make_figure;
mod node_locations;
mod rand_utils;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt)]
struct Arguments {
    file: PathBuf,
    #[structopt(default_value = "1")]
    count: usize,
}

#[derive(Deserialize)]
enum SaveFormat {
    #[serde(rename = "yaml")]
    SaveYaml(String),
    #[serde(rename = "json")]
    SaveJson(String),
}
#[derive(Deserialize)]
enum PlotFormat {
    #[serde(rename = "eps")]
    PlotEps,
    #[serde(rename = "png")]
    PlotPng,
}

#[derive(Deserialize)]
struct PlotInfo {
    format: PlotFormat,
    width: u32,
    height: u32,
    name: String,
    title: Option<String>,
}

const DEFAULT_TRIALS: usize = 100;

#[derive(Deserialize)]
struct Configuration {
    width: f64,
    height: f64,
    origin_distance: f64,
    points_distance: f64,
    lines: Vec<usize>,
    trials: Option<usize>,
    save_option: Option<SaveFormat>,
    plot_option: Option<PlotInfo>,
}

impl Configuration {
    fn make_factory_config(&self) -> bezier_point_factory::FactoryConfig {
        bezier_point_factory::FactoryConfig {
            center_radius: self.origin_distance,
            point_radius: self.points_distance,
            size_x: self.width,
            size_y: self.height,
        }
    }

    fn set_defaults(mut self) -> Self {
        self.trials.get_or_insert(DEFAULT_TRIALS);
        self
    }
}

struct TrialCounter {
    count: usize,
}

impl TrialCounter {
    fn new(count: Option<usize>) -> Self {
        let count = count.unwrap_or(DEFAULT_TRIALS);
        Self { count }
    }

    fn run(&mut self) -> bool {
        println!("{}", self.count);
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
    }
}

fn try_build_network(
    bpf: &mut bezier_point_factory::BezierPointFactory,
    lines: &[usize],
) -> Option<build_graph::Network> {
    let curves = make_curves::make_curves(bpf, lines.len());
    let inter = intersections::make_intersection_lists(&curves)?;
    let nodes = node_locations::generate_node_lists(inter.direct_intersections, lines);
    build_graph::build_graph(&curves, &nodes, &inter.inverse_intersections)
}

fn build_network(
    factory_config: &bezier_point_factory::FactoryConfig,
    trials: Option<usize>,
    lines: &[usize],
) -> Option<build_graph::Network> {
    let mut bezier_points_factory = bezier_point_factory::BezierPointFactory::new(factory_config);
    let mut trials = TrialCounter::new(trials);
    while trials.run() {
        if let Some(output) = try_build_network(&mut bezier_points_factory, lines) {
            return Some(output);
        }
        bezier_points_factory.reset();
    }
    None
}

fn load_config(f: PathBuf) -> MResult<Configuration> {
    let file = File::open(f)?;
    let conf = serde_yaml::from_reader(file)?;
    Ok(conf)
}

fn serialize<T: Serialize, E, F>(base: &str, id: usize, ext: &str, t: &T, f: F) -> MResult<()>
where
    F: Fn(File, &T) -> Result<(), E>,
    E: std::error::Error + 'static,
{
    let file_name = format!("{base}-{id}.{ext}");
    let file = File::create(file_name)?;
    f(file, t)?;
    Ok(())
}

fn save_if_required(
    net: &build_graph::Network,
    format: &Option<SaveFormat>,
    id: usize,
) -> MResult<()> {
    if let Some(format) = format {
        match format {
            SaveFormat::SaveJson(name) => serialize(name, id, "json", net, serde_json::to_writer)?,
            SaveFormat::SaveYaml(name) => serialize(name, id, "yaml", net, serde_yaml::to_writer)?,
        };
    }
    Ok(())
}

fn plot<F>(name: &str, id: usize, ext: &str, width: u32, height: u32, mut f: F) -> MResult<()>
where
    F: FnMut(&str, u32, u32) -> Result<(), gnuplot::GnuplotInitError>,
{
    let name = format!("{name}-{id}.{ext}");
    f(&name, width, height)?;
    Ok(())
}

fn plot_if_required(
    net: &build_graph::Network,
    format: &Option<PlotInfo>,
    id: usize,
) -> MResult<()> {
    if let Some(format) = format {
        let mut fig = make_figure::make_figure(net);
        if let Some(title) = &format.title {
            fig.set_title(title);
        }

        match format.format {
            PlotFormat::PlotEps => plot(
                &format.name,
                id,
                "eps",
                format.width,
                format.height,
                |n, w, h| fig.save_to_eps(n, w, h),
            )?,
            PlotFormat::PlotPng => plot(
                &format.name,
                id,
                "png",
                format.width,
                format.height,
                |n, w, h| fig.save_to_png(n, w, h),
            )?,
        }
    }
    Ok(())
}

fn main() -> MResult<()> {
    let args = Arguments::from_args();
    let config = load_config(args.file)?.set_defaults();

    for i in 0..args.count {
        let factory_config = config.make_factory_config();
        if let Some(network) = build_network(&factory_config, config.trials, &config.lines) {
            save_if_required(&network, &config.save_option, i)?;
            plot_if_required(&network, &config.plot_option, i)?;
        } else {
            println!(
                "Error: system did not generate a Connected Random Network in {} trials",
                config.trials.unwrap()
            );
        }
    }

    Ok(())
}
