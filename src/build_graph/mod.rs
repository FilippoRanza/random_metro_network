use petgraph::graph;
use serde::Serialize;

use crate::Curve;
use crate::float_table::FloatMatrix;

mod lines_to_petgraph;
mod build_lines;
mod point_factory;


#[derive(Debug, Serialize)]
pub struct Network {
    pub lines: Vec<Vec<usize>>,
    pub points: Vec<(f64, f64)>,
    pub graph: NetGraph,
}

pub type NetGraph = graph::UnGraph<usize, f64>;
type Lines = Vec<Vec<usize>>;
type Pt = (f64, f64);


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

