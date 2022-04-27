use super::Pt;
use crate::float_table::FloatMatrix;
use crate::Curve;
use flo_curves::{BezierCurve, Coordinate};


pub struct PointListFactory {
    points: Vec<Pt>,
    inter_table: FloatMatrix<usize>,
}

impl PointListFactory {
    pub fn new(lines: usize) -> Self {
        let inter_table = FloatMatrix::new(lines);
        Self {
            points: vec![],
            inter_table,
        }
    }

    pub fn add_point(&mut self, id: usize, c: &Curve, t: f64) -> usize {
        let curr = self.points.len();
        let pt = get_point(c, t);
        self.points.push(pt);
        self.inter_table.insert(id, t, curr);
        curr
    }

    pub fn get_index_for_intersection(&self, id: usize, t: f64) -> Option<usize> {
        self.inter_table.get(id, t).copied()
    }

    pub fn get_points(self) -> Vec<Pt> {
        self.points
    }
}


fn get_point(c: &Curve, t: f64) -> (f64, f64) {
    let p = c.point_at_pos(t);
    (p.get(0), p.get(1))
}

