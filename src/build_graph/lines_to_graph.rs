use super::check_connected_graph;
use super::{point_factory::PointListFactory, Lines, NetGraph, Network, Pt};
use simplegraph::Graph;

pub fn build_graph(pts: PointListFactory, lines: Lines) -> Option<Network> {
    let points = pts.get_points();
    is_connected(&points, &lines).map(|graph| Network {
        lines,
        points,
        graph,
    })
}

fn is_connected(pts: &[Pt], lines: &[Vec<usize>]) -> Option<NetGraph> {
    let graph = line_to_graph(pts, lines);

    if check_connected_graph::is_connected(&graph) {
        Some(add_arc_weights(graph, pts))
    } else {
        None
    }
}

fn line_to_graph(pts: &[Pt], lines: &[Vec<usize>]) -> NetGraph {
    let net_graph = lines.iter().fold(init_graph(pts), |graph, line| {
        add_line_to_graph(graph, line)
    });
    net_graph
}

fn init_graph(pts: &[Pt]) -> NetGraph {
    let nodes = pts.len();
    NetGraph::new_undirect(nodes)
}

fn add_line_to_graph(mut net_graph: NetGraph, line: &[usize]) -> NetGraph {
    for (a, b) in SuccessorIterator::new(line).map(|(a, b)| (*a, *b)) {
        net_graph.add_new_default_arc(a, b);
    }
    net_graph
}

fn add_arc_weights(mut net_graph: NetGraph, pts: &[Pt]) -> NetGraph {
    net_graph.update_all_arcs_weight(|i, j, _| distance(pts, i, j));
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
