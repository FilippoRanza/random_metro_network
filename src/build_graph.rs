use crate::float_table::FloatMatrix;
use crate::Curve;
use flo_curves::{BezierCurve, Coordinate};
use petgraph::{algo, graph};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Network {
    pub lines: Vec<Vec<usize>>,
    pub points: Vec<(f64, f64)>,
    pub graph: NetGraph,
}

pub type NetGraph = graph::UnGraph<usize, f64>;

pub fn build_graph(
    curves: &[Curve],
    nodes: &[Vec<f64>],
    intersections: &FloatMatrix<(usize, f64)>,
) -> Option<Network> {
    let mut lines = vec![vec![]; curves.len()];
    let mut point_factory = PointListFactory::new(curves.len());

    for (i, (c, n)) in curves.iter().zip(nodes.iter()).enumerate() {
        for t in n {
            let idx = get_point_index(i, c, *t, &mut point_factory, intersections);
            lines[i].push(idx);
        }
    }
    let points = point_factory.points;
    if let Some(graph) = is_connected(&points, &lines) {
        Some(Network {
            lines,
            points,
            graph,
        })
    } else {
        None
    }
}

fn get_point_index(
    line_id: usize,
    c: &Curve,
    t: f64,
    factory: &mut PointListFactory,
    intersections: &FloatMatrix<(usize, f64)>,
) -> usize {
    if let Some((j, tj)) = intersections.get(line_id, t) {
        if let Some(idx) = factory.get_index_for_intersection(*j, *tj) {
            idx
        } else {
            factory.add_point(line_id, c, t)
        }
    } else {
        factory.add_point(line_id, c, t)
    }
}

type Pt = (f64, f64);

struct PointListFactory {
    points: Vec<Pt>,
    inter_table: FloatMatrix<usize>,
}

impl PointListFactory {
    fn new(lines: usize) -> Self {
        let inter_table = FloatMatrix::new(lines);
        Self {
            points: vec![],
            inter_table,
        }
    }

    fn add_point(&mut self, id: usize, c: &Curve, t: f64) -> usize {
        let curr = self.points.len();
        let pt = get_point(c, t);
        self.points.push(pt);
        self.inter_table.insert(id, t, curr);
        curr
    }

    fn get_index_for_intersection(&self, id: usize, t: f64) -> Option<usize> {
        self.inter_table.get(id, t).copied()
    }
}

fn get_point(c: &Curve, t: f64) -> (f64, f64) {
    let p = c.point_at_pos(t);
    (p.get(0), p.get(1))
}

fn is_connected(pts: &[Pt], lines: &[Vec<usize>]) -> Option<NetGraph> {
    let graph = line_to_graph(pts, lines);
    let conn_comps = dbg! {algo::connected_components(&graph)};
    if conn_comps == 1 {
        Some(add_arc_weights(graph, pts))
    } else {
        None
    }
}

fn line_to_graph(pts: &[Pt], lines: &[Vec<usize>]) -> NetGraph {
    let net_graph = lines.iter().fold(init_graph(pts, lines), add_line_to_graph);
    net_graph
}

fn init_graph(pts: &[Pt], lines: &[Vec<usize>]) -> NetGraph {
    let nodes = pts.len();
    let arcs = arc_count(lines);
    NetGraph::with_capacity(nodes, arcs)
}

fn add_line_to_graph(mut net_graph: NetGraph, line: &Vec<usize>) -> NetGraph {
    net_graph.extend_with_edges(SuccessorIterator::new(line).map(|(a, b)| (*a as u32, *b as u32)));
    net_graph
}

fn arc_count(lines: &[Vec<usize>]) -> usize {
    lines.iter().map(|l| l.len() - 1).sum()
}

fn add_arc_weights(mut net_graph: NetGraph, pts: &[Pt]) -> NetGraph {
    for edge in net_graph.edge_indices() {
        if let Some((a, b)) = net_graph.edge_endpoints(edge) {
            let i = a.index();
            let j = b.index();
            let d = distance(pts, i, j);
            *net_graph.edge_weight_mut(edge).unwrap() = d;
        }
    }

    for (id, node_w) in net_graph.node_weights_mut().enumerate() {
        *node_w = id;
    } 

    net_graph
}

fn distance(pts: &[Pt], i: usize, j: usize) -> f64 {
    let (x1, y1) = pts[i];
    let (x2, y2) = pts[j];
    let dx = x1 - x2;
    let dy = y1 - y2;
    ((dx * dx) + (dy * dy)).sqrt()
}

struct SuccessorIterator<'a, T> {
    prev: Option<&'a T>,
    iter: std::slice::Iter<'a, T>,
}

impl<'a, T> SuccessorIterator<'a, T> {
    fn new(coll: &'a [T]) -> Self {
        Self {
            prev: None,
            iter: coll.iter(),
        }
    }
}

impl<'a, T> Iterator for SuccessorIterator<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let prev = if let Some(prev) = self.prev {
            prev
        } else {
            self.iter.next()?
        };
        let curr = self.iter.next()?;
        let output = (prev, curr);
        self.prev = Some(curr);
        Some(output)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_successor_iterator() {
        let vect = [1, 2, 3, 4, 5, 6];
        let result: Vec<(&usize, &usize)> = SuccessorIterator::new(&vect).collect();
        let expected = vec![(&1, &2), (&2, &3), (&3, &4), (&4, &5), (&5, &6)];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_check_connection() {
        let (pts, lines) = lines_to_net(
            vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![4, 6, 7, 2, 8],
                vec![6, 9, 10, 11],
                vec![12, 13, 9, 14],
            ],
            15,
        );
        assert!(is_connected(&pts, &lines).is_some());

        let (pts, lines) = lines_to_net(
            vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![4, 6, 7, 2, 8],
                vec![6, 9, 10, 11],
                vec![12, 13, 14, 15],
            ],
            16,
        );
        assert!(is_connected(&pts, &lines).is_none());
    }

    fn lines_to_net(lines: Vec<Vec<usize>>, count: usize) -> (Vec<Pt>, Vec<Vec<usize>>) {
        (vec![(0., 0.); count], lines)
    }
}
