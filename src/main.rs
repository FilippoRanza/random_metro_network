use flo_curves::bezier;
use flo_curves::Coord2;

use simplegraph::dot;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

type Curve = bezier::Curve<Coord2>;

mod all_direct_path;
mod bezier_point_factory;
mod build_graph;
mod float_table;
mod intersections;
mod make_curves;
mod node_locations;
mod rand_utils;
mod station_wait_times;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt)]
struct Arguments {
    file: PathBuf,
}

#[derive(Deserialize)]
enum SaveFormat {
    #[serde(rename = "yaml")]
    SaveYaml(String),
    #[serde(rename = "json")]
    SaveJson(String),
}

const DEFAULT_TRIALS: usize = 100;
const DEFAULT_COUNT: usize = 1;

fn get_default_trials() -> usize {
    DEFAULT_TRIALS
}

fn get_default_count() -> usize {
    DEFAULT_COUNT
}

#[derive(Deserialize)]
struct Configuration {
    width: f64,
    height: f64,
    origin_distance: f64,
    points_distance: f64,
    lines: Vec<usize>,
    #[serde(default = "get_default_trials")]
    trials: usize,
    #[serde(default = "get_default_count")]
    count: usize,
    save_option: Option<SaveFormat>,
    export_graph: Option<String>,
    station_wait: Option<station_wait_times::StationWaitTimeConfig>,
    all_direct_path: Option<bool>,
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
}

struct TrialCounter {
    count: usize,
}

impl TrialCounter {
    fn new(count: usize) -> Self {
        Self { count }
    }

    fn run(&mut self) -> bool {
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
    build_graph::build_network(&curves, &nodes, &inter.inverse_intersections)
}

fn build_network(
    factory_config: &bezier_point_factory::FactoryConfig,
    trials: usize,
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

fn mk_file_name(base: &str, id: usize, ext: &str) -> String {
    format!("{base}-{id}.{ext}")
}

fn serialize<T: Serialize, E, F>(base: &str, id: usize, ext: &str, t: &T, f: F) -> MResult<()>
where
    F: Fn(File, &T) -> Result<(), E>,
    E: std::error::Error + 'static,
{
    let file_name = mk_file_name(base, id, ext);
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

fn export_if_required(
    net: &build_graph::Network,
    base_name: &Option<String>,
    id: usize,
) -> MResult<()> {
    if let Some(base_name) = &base_name {
        let file_name = mk_file_name(base_name, id, "dot");
        let mut file = File::create(file_name)?;
        write!(file, "{}", dot::to_dot_source(&net.graph))?;
    }

    Ok(())
}

fn apply_station_wait_if_required(
    net: build_graph::Network,
    conf: &Option<station_wait_times::StationWaitTimeConfig>,
) -> MResult<build_graph::Network> {
    match conf {
        Some(conf) => station_wait_times::add_wait_time(net, conf),
        None => Ok(net),
    }
}

fn apply_all_direct_path_is_required(
    net: build_graph::Network,
    conf: &Option<bool>,
) -> build_graph::Network {
    let conf = conf.unwrap_or(false);
    if conf {
        all_direct_path::all_direct_path(net)
    } else {
        net
    }
}

fn build_random_instance(config: &Configuration, id: usize) -> MResult<()> {
    let factory_config = config.make_factory_config();

    if let Some(network) = build_network(&factory_config, config.trials, &config.lines) {
        let network = apply_station_wait_if_required(network, &config.station_wait)?;
        let network = apply_all_direct_path_is_required(network, &config.all_direct_path);
        save_if_required(&network, &config.save_option, id)?;
        export_if_required(&network, &config.export_graph, id)?;
    } else {
        println!(
            "Error: system did not generate a Connected Random Network in {} trials",
            config.trials
        );
    }

    Ok(())
}

fn main() -> MResult<()> {
    let args = Arguments::from_args();
    let config = load_config(args.file)?;

    for i in 0..config.count {
        build_random_instance(&config, i)?;
    }

    Ok(())
}
