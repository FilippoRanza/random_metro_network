use flo_curves::bezier;
use flo_curves::Coord2;

use serde::Deserialize;
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
}

#[derive(Deserialize)]
struct Configuration {
    width: f64,
    height: f64,
    origin_distance: f64,
    points_distance: f64,
    lines: Vec<usize>,
}

fn load_config(f: PathBuf) -> MResult<Configuration> {
    let file = File::open(f)?;
    let conf = serde_yaml::from_reader(file)?;
    Ok(conf)
}

fn main() -> MResult<()> {
    let args = Arguments::from_args();
    let config = load_config(args.file)?;

    let factory_config = bezier_point_factory::FactoryConfig {
        center_radius: config.origin_distance,
        point_radius: config.points_distance,
        size_x: config.width,
        size_y: config.height,
    };
    let mut bezier_points_factory = bezier_point_factory::BezierPointFactory::new(&factory_config);

    loop {
        let curves = make_curves::make_curves(&mut bezier_points_factory, config.lines.len());
        if let Some(inter) = intersections::make_intersection_lists(&curves) {
            let nodes =
                node_locations::generate_node_lists(inter.direct_intersections, config.lines);
            let network = build_graph::build_graph(&curves, &nodes, &inter.inverse_intersections);
            if let Some(network) = network {
                let mut fig = make_figure::make_figure(&network);
                fig.show().unwrap();
            } else {
                println!("Network is not connectd");
            }
            break;
        }
    }

    Ok(())
}
