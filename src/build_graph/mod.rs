use serde::Serialize;
use simplegraph::AdjList;

use crate::float_table::FloatMatrix;
use crate::Curve;

mod build_lines;
mod check_connected_graph;
mod lines_to_graph;
mod point_factory;

#[derive(Debug, Serialize)]
pub struct Network {
    pub lines: Lines,
    pub points: Vec<Pt>,
    pub graph: NetGraph,
}

pub type NetGraph = AdjList<f64>;
pub type Lines = Vec<Vec<usize>>;
pub type Pt = (f64, f64);

fn new_lines(line_count: usize) -> Lines {
    vec![vec![]; line_count]
}

pub fn build_network(
    curves: &[Curve],
    nodes: &[Vec<f64>],
    intersections: &FloatMatrix<(usize, f64)>,
) -> Option<Network> {
    let (point_factory, lines) = build_lines::build_lines(curves, nodes, intersections);
    lines_to_graph::build_graph(point_factory, lines)
}
