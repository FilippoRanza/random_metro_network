use petgraph::graph;
use serde::Serialize;

use crate::float_table::FloatMatrix;
use crate::Curve;

mod build_lines;
mod lines_to_petgraph;
mod point_factory;

#[derive(Debug, Serialize)]
pub struct Network {
    pub lines: Lines,
    pub points: Vec<Pt>,
    pub graph: NetGraph,
}

pub type NetGraph = graph::UnGraph<usize, f64>;
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
    lines_to_petgraph::build_graph(point_factory, lines)
}
