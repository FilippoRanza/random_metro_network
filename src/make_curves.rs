use crate::bezier_point_factory;
use crate::Curve;
use flo_curves::{bezier, BezierCurveFactory};

pub fn make_curves(pf: &mut bezier_point_factory::BezierPointFactory, count: usize) -> Vec<Curve> {
    (0..count).map(|_| make_curve(pf)).collect()
}

fn make_curve(point_factory: &mut bezier_point_factory::BezierPointFactory) -> Curve {
    let points = point_factory.get_bezier_points();
    bezier::Curve::from_points(
        points.start.into(),
        (points.ctrl_1.into(), points.ctrl_2.into()),
        points.end.into(),
    )
}
