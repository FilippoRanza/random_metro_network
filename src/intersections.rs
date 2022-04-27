use crate::float_table::FloatMatrix;
use crate::Curve;
use flo_curves::bezier;

#[derive(Debug)]
pub struct Intersections {
    pub direct_intersections: Vec<Vec<f64>>,
    pub inverse_intersections: FloatMatrix<(usize, f64)>,
}

pub fn make_intersection_lists(curves: &[Curve]) -> Option<Intersections> {
    let mut output = find_all_intersections(curves);
    sort_all(&mut output.direct_intersections)?;
    Some(output)
}

fn find_all_intersections(curves: &[Curve]) -> Intersections {
    let mut direct_intersections = vec![vec![]; curves.len()];
    let mut inverse_intersections = FloatMatrix::new(curves.len());
    all_cross_iterator(curves, |c1, c2| {
        push_intersections(
            &mut direct_intersections,
            &mut inverse_intersections,
            c1,
            c2,
        );
    });

    Intersections {
        direct_intersections,
        inverse_intersections,
    }
}

type IndexCurve<'a> = (usize, &'a Curve);
fn push_intersections<'a>(
    direct_intersections: &mut [Vec<f64>],
    inverse_intersections: &mut FloatMatrix<(usize, f64)>,
    c1: IndexCurve<'a>,
    c2: IndexCurve<'a>,
) {
    let (i, c1) = c1;
    let (j, c2) = c2;
    let inters = bezier::curve_intersects_curve_clip(c1, c2, 1e-6);
    for (t1, t2) in inters {
        direct_intersections[i].push(t1);
        direct_intersections[j].push(t2);
        inverse_intersections.insert(j, t2, (i, t1));
        inverse_intersections.insert(i, t1, (j, t1));
    }
}

fn sort_all(lists: &mut Vec<Vec<f64>>) -> Option<()> {
    for list in lists {
        if list.is_empty() {
            return None;
        }
        list.push(0.);
        list.push(1.);
        list.sort_by(|a, b| cmp_f64(*a, *b));
    }
    Some(())
}

fn cmp_f64(a: f64, b: f64) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn all_cross_iterator<F, T>(slice: &[T], mut f: F)
where
    F: FnMut((usize, &T), (usize, &T)),
{
    for (i, c1) in slice.iter().enumerate() {
        for (j, c2) in slice[i + 1..].iter().enumerate() {
            let j = i + j + 1;
            f((i, c1), (j, c2));
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sort_f64_vec() {
        let mut vec = vec![0.45, 0.12, 0.56, 0.89, 0.23];
        vec.sort_by(|a, b| cmp_f64(*a, *b));
        let result = vec![0.12, 0.23, 0.45, 0.56, 0.89];
        assert_eq!(result, vec);
    }

    #[test]
    fn test_all_cross_iter() {
        let list = ['a', 'b', 'c'];
        let mut result = vec![];
        all_cross_iterator(&list, |(i, a), (j, b)| result.push((i, j, *a, *b)));
        let expected = vec![(0, 1, 'a', 'b'), (0, 2, 'a', 'c'), (1, 2, 'b', 'c')];
        assert_eq!(result, expected);
    }
}
