pub type Point = (f64, f64);

use crate::rand_utils::random_in_range;

pub struct BezierPointFactory {
    orig_factory: OriginPointFactory,
}

impl BezierPointFactory {
    pub fn new(config: &FactoryConfig) -> Self {
        let orig_factory = OriginPointFactory::new(
            config.center_radius,
            config.point_radius,
            config.size_x,
            config.size_y,
        );
        Self { orig_factory }
    }

    pub fn get_bezier_points(&mut self) -> BezierPoints {
        let start = self.orig_factory.next_point();
        let end = self.orig_factory.next_point();
        let ctl_factory = ControlPointFactory::new(start, end);
        let ctrl_1 = ctl_factory.next_point();
        let ctrl_2 = ctl_factory.next_point();
        BezierPoints {
            start,
            end,
            ctrl_1,
            ctrl_2,
        }
    }
}

pub struct BezierPoints {
    pub start: Point,
    pub end: Point,
    pub ctrl_1: Point,
    pub ctrl_2: Point,
}

pub struct FactoryConfig {
    pub center_radius: f64,
    pub point_radius: f64,
    pub size_x: f64,
    pub size_y: f64,
}

struct OriginPointFactory {
    center_radius: f64,
    point_radius: f64,
    size_x: f64,
    size_y: f64,
    points: Vec<Point>,
}

impl OriginPointFactory {
    fn new(center_radius: f64, point_radius: f64, size_x: f64, size_y: f64) -> Self {
        Self {
            center_radius,
            point_radius,
            size_x,
            size_y,
            points: vec![],
        }
    }

    fn next_point(&mut self) -> Point {
        loop {
            let pt = self.rand_point();
            if self.valid_point(pt) {
                self.points.push(pt);
                return pt;
            }
        }
    }

    fn rand_point(&self) -> Point {
        let x_range = (-self.size_x, self.size_x);
        let y_range = (-self.size_y, self.size_y);
        rand_point(x_range, y_range)
    }

    fn valid_point(&self, pt: Point) -> bool {
        self.accept_radius(pt) && self.accept_position(pt)
    }

    fn accept_radius(&self, pt: Point) -> bool {
        accept_radius(pt, self.center_radius)
    }

    fn accept_position(&self, pt: Point) -> bool {
        self.points
            .iter()
            .all(|ot| accpet_distance(pt, *ot, self.point_radius))
    }
}

struct ControlPointFactory {
    x_range: (f64, f64),
    y_range: (f64, f64),
}

impl ControlPointFactory {
    fn new(p1: Point, p2: Point) -> Self {
        let (x_range, y_range) = get_bounds(p1, p2);
        Self { x_range, y_range }
    }

    fn next_point(&self) -> Point {
        rand_point(self.x_range, self.y_range)
    }
}

fn get_bounds(p1: (f64, f64), p2: (f64, f64)) -> ((f64, f64), (f64, f64)) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let x_range = min_max((x1, x2));
    let y_range = min_max((y1, y2));
    (x_range, y_range)
}

fn min_max(t: (f64, f64)) -> (f64, f64) {
    let (a, b) = t;
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn accpet_distance(p1: Point, p2: Point, radius: f64) -> bool {
    let dist = euclid_distance(p1, p2);
    let arg = -(dist / radius);
    let p = arg.exp();
    fastrand::f64() > p
}

fn accept_radius(pt: Point, radius: f64) -> bool {
    accpet_distance(pt, (0., 0.), radius)
}

fn euclid_distance(p1: Point, p2: Point) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let x_dist = x1 - x2;
    let y_dist = y1 - y2;
    let tmp = (x_dist * x_dist) + (y_dist * y_dist);
    tmp.sqrt()
}

fn rand_point(range_x: (f64, f64), range_y: (f64, f64)) -> (f64, f64) {
    let x = random_in_range(range_x);
    let y = random_in_range(range_y);
    (x, y)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_euclid_distance() {
        let pt_1 = (0., 3.);
        let pt_2 = (4., 0.);
        let dist = euclid_distance(pt_1, pt_2);
        assert_eq!(dist, 5.);
    }

    #[test]
    fn test_get_bounds() {
        let p1 = (40., 100.);
        let p2 = (90., 50.);

        let (x_range, y_range) = get_bounds(p1, p2);
        assert_eq!(x_range, (40., 90.));
        assert_eq!(y_range, (50., 100.));
    }

    #[test]
    fn test_min_max() {
        let test_1 = (6.7, 13.2);
        let res = min_max(test_1);
        assert_eq!(res, (6.7, 13.2));

        let test_1 = (13.2, 6.7);
        let res = min_max(test_1);
        assert_eq!(res, (6.7, 13.2));
    }
}
