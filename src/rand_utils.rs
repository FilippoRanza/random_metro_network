pub fn random_in_range(range: (f64, f64)) -> f64 {
    let (min, max) = range;
    let delta = max - min;
    let r = fastrand::f64();
    (r * delta) + min
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_random_in_range() {
        let min = -5.6;
        let max = 6.7;
        let range = (min, max);
        for _ in 0..10000 {
            let rnd = random_in_range(range);
            assert!(rnd >= min && rnd <= max);
        }
    }
}
