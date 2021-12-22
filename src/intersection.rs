type Coord = (f64, f64);
type Segment = (Coord, Coord);

#[derive(PartialEq, Debug)]
pub enum Intersection {
    Point(Coord),
    Empty,
    Parallel,
}

pub fn intersection(s1: Segment, s2: Segment) -> Intersection {
    let den = dbg! {denominator(s1, s2)};
    if is_zero(den) {
        return Intersection::Parallel;
    }
    let l1 = dbg! {l1_parameter(s1, s2)};
    if dbg! {is_outside(l1, den)} {
        return Intersection::Empty;
    }
    let l2 = l2_parameter(s1, s2);
    if dbg! {is_outside(l2, den)} {
        return Intersection::Empty;
    }

    let t = l1 / den;
    let x = s1.0 .0 + t * (s1.1 .0 - s1.0 .0);
    let y = s1.0 .1 + t * (s1.1 .1 - s1.0 .1);
    Intersection::Point((x, y))
}

fn l1_parameter(s1: Segment, s2: Segment) -> f64 {
    let ((x1, y1), _) = s1;
    let ((x3, y3), (x4, y4)) = s2;
    ((x1 - x3) * (y3 - y4)) - ((y1 - y3) * (x3 - x4))
}

fn l2_parameter(s1: Segment, s2: Segment) -> f64 {
    let ((x1, y1), (x2, y2)) = s1;
    let ((x3, y3), _) = s2;
    ((x1 - x3) * (y1 - y2)) - ((y1 - y3) * (x1 - x2))
}

fn denominator(s1: Segment, s2: Segment) -> f64 {
    let ((x1, y1), (x2, y2)) = s1;
    let ((x3, y3), (x4, y4)) = s2;
    ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4))
}

fn is_zero(n: f64) -> bool {
    n.abs() < 1e-12
}

fn is_outside(p: f64, den: f64) -> bool {
    let param = p / den;
    param < 0. || param > 1.
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_intersection() {
        let seg_a = ((1., 1.), (3., 2.));
        let seg_b = ((1., 4.), (2., -1.));
        assert_eq!(
            intersection(seg_a, seg_b),
            Intersection::Point((17. / 11., 14. / 11.))
        );
    }

    #[test]
    fn test_parallel() {
        let seg_a = ((1., 1.), (2., 3.));
        let seg_b = ((2., 2.), (3., 4.));
        assert_eq!(intersection(seg_a, seg_b), Intersection::Parallel);

        let seg_a = ((1., 1.), (2., 3.));
        let seg_b = ((-4., -4.), (-3., -2.));
        assert_eq!(intersection(seg_a, seg_b), Intersection::Parallel);
    }

    #[test]
    fn test_empty_intesection() {
        let seg_a = ((1., 1.), (2., 3.));
        let seg_b = ((4., 2.), (3., 4.));
        assert_eq!(intersection(seg_a, seg_b), Intersection::Empty);

        let seg_a = ((3., 2.), (6., 1.));
        let seg_b = ((5., 2.), (7., 5.));
        assert_eq!(intersection(seg_a, seg_b), Intersection::Empty);
    }
}
