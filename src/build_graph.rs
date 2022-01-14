use crate::float_table::FloatMatrix;
use crate::Curve;
use flo_curves::{BezierCurve, Coordinate};

#[derive(Debug)]
pub struct Network {
    pub lines: Vec<Vec<usize>>,
    pub points: Vec<(f64, f64)>,
}

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

    let net = Network {
        lines,
        points: point_factory.points,
    };
    if is_connected(&net) {
        Some(net)
    } else {
        None
    }
}

fn get_point_index(
    id: usize,
    c: &Curve,
    t: f64,
    factory: &mut PointListFactory,
    intersections: &FloatMatrix<(usize, f64)>,
) -> usize {
    if let Some((j, tj)) = intersections.get(id, t) {
        if let Some(idx) = factory.get_index_for_intersection(*j, *tj) {
            idx
        } else {
            factory.add_point(id, c, t)
        }
    } else {
        factory.add_point(id, c, t)
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

fn is_connected(net: &Network) -> bool {
    let graph = make_graph(&net.lines, net.points.len());
    let visited = breadth_first_search(&graph);
    visited.into_iter().all(|b| b)
}

fn make_graph(lines: &[Vec<usize>], node_count: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; node_count];
    for line in lines {
        insert_line(line, &mut adj_list);
    }
    adj_list
}

fn insert_line(line: &[usize], g: &mut Vec<Vec<usize>>) {
    let mut prev: Option<usize> = None;
    for n in line {
        if let Some(prev) = prev {
            g[*n].push(prev);
            g[prev].push(*n);
        }
        prev = Some(*n);
    }
}

fn breadth_first_search(g: &[Vec<usize>]) -> Vec<bool> {
    let mut visited = vec![false; g.len()];
    let mut stack: Vec<usize> = vec![0];
    visited[0] = true;
    while let Some(curr) = stack.pop() {
        for next in &g[curr] {
            if !visited[*next] {
                visited[*next] = true;
                stack.push(*next);
            }
        }
    }
    visited
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_bfs() {
        let connected_graph = vec![
            vec![1],
            vec![2, 3, 4],
            vec![1, 3],
            vec![1, 2],
            vec![5, 6],
            vec![4],
            vec![4, 7],
            vec![6],
        ];
        let expected = vec![true; connected_graph.len()];
        let result = breadth_first_search(&connected_graph);
        assert_eq!(expected, result);

        let unconnected_graph = vec![
            vec![1],
            vec![2, 3],
            vec![1, 3],
            vec![1, 2],
            vec![5, 6],
            vec![4],
            vec![4, 7],
            vec![6],
        ];
        let result = breadth_first_search(&unconnected_graph);
        let expected = vec![true, true, true, true, false, false, false, false];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_to_graph() {
        let lines = vec![vec![0, 1, 2, 3, 4], vec![5, 6, 1, 7]];
        let graph = make_graph(&lines, 8);
        let correct = vec![
            vec![1],
            vec![0, 2, 6, 7],
            vec![1, 3],
            vec![2, 4],
            vec![3],
            vec![6],
            vec![5, 1],
            vec![1],
        ];
        assert_eq!(correct, graph);
    }

    #[test]
    fn test_check_connection() {
        let connect_net = lines_to_net(
            vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![4, 6, 7, 2, 8],
                vec![6, 9, 10, 11],
                vec![12, 13, 9, 14],
            ],
            15,
        );
        assert!(is_connected(&connect_net));

        let not_connect_net = lines_to_net(
            vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![4, 6, 7, 2, 8],
                vec![6, 9, 10, 11],
                vec![12, 13, 14, 15],
            ],
            16,
        );
        assert!(!is_connected(&not_connect_net));
    }

    fn lines_to_net(lines: Vec<Vec<usize>>, count: usize) -> Network {
        Network {
            lines,
            points: vec![(0., 0.); count],
        }
    }
}
