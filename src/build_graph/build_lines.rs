
use crate::Curve;
use crate::float_table::FloatMatrix;

use super::{Lines, point_factory::PointListFactory};
use super::{new_lines};

pub fn build_lines(
    curves: &[Curve],
    nodes: &[Vec<f64>],
    intersections: &FloatMatrix<(usize, f64)>,
) -> (PointListFactory, Lines) {
    let mut lines = new_lines(curves.len());
    let mut point_factory = PointListFactory::new(curves.len());

    for (i, (c, n)) in curves.iter().zip(nodes.iter()).enumerate() {
        for t in n {
            let idx = get_point_index(i, c, *t, &mut point_factory, intersections);
            lines[i].push(idx);
        }
    }
    (point_factory, lines)
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

