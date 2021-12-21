use rand::prelude::*;

pub fn rand_normal(mu: f64, sigma: f64) -> f64 {
    let mut rng = thread_rng();
    let normal = rand_distr::Normal::new(mu, sigma).unwrap();
    normal.sample(&mut rng)
}

pub fn random_in(f: f64) -> f64 {
    (rand_f64() - 0.5) * f
}

pub fn random_below(a: f64) -> f64 {
    rand_f64() * a
}

pub fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}
