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
) -> Network {
    let mut lines = vec![vec![]; curves.len()];
    let mut point_factory = PointListFactory::new(curves.len());

    for (i, (c, n)) in curves.iter().zip(nodes.iter()).enumerate() {
        for t in n {
            let idx = get_point_index(i, c, *t, &mut point_factory, intersections);
            lines[i].push(idx);
        }
    }

    Network {
        lines,
        points: point_factory.points,
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
