pub fn generate_node_lists(inters: Vec<Vec<f64>>, counts: Vec<usize>) -> Vec<Vec<f64>> {
    inters
        .into_iter()
        .zip(counts.into_iter())
        .map(|(i, c)| generate_node(i, c))
        .collect()
}

fn generate_node(inter: Vec<f64>, count: usize) -> Vec<f64> {
    let rem = count - inter.len();
    let mut output = Vec::with_capacity(count);
    let rem = rem as f64;
    let mut prev = None;
    for curr in inter {
        if let Some(prev) = prev {
            let len = (curr - prev) * rem;
            SubdivisionIterator::new((prev, curr), len).for_each(|n| output.push(n));
            output.push(curr);
        }
        prev = Some(curr);
    }

    output
}

struct SubdivisionIterator {
    curr: f64,
    delta: f64,
    end: f64,
}

impl SubdivisionIterator {
    fn new(interval: (f64, f64), count: f64) -> Self {
        let (begin, end) = interval;
        let delta = (end - begin) / count;
        let curr = begin + delta;
        Self { curr, delta, end }
    }
}

impl Iterator for SubdivisionIterator {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.end {
            let tmp = self.curr;
            self.curr += self.delta;
            Some(tmp)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_subdivde() {
        let interval = (0., 1.);
        let count = 4.;
        let result = subdivide(interval, count);
        let expected = vec![0.25, 0.5, 0.75];
        assert_eq!(result, expected);

        let interval = (1.5, 2.5);
        let count = 4.;
        let result = subdivide(interval, count);
        let expected = vec![1.75, 2., 2.25];
        assert_eq!(result, expected);
    }

    fn subdivide(interval: (f64, f64), count: f64) -> Vec<f64> {
        let subdivisions = SubdivisionIterator::new(interval, count);
        subdivisions.collect()
    }
}
