pub fn random_in(f: f64) -> f64 {
    (fastrand::f64() - 0.5) * f
}

pub fn random_within(min: f64, width: f64) -> f64 {
    random_below(width) + min
}

pub fn random_below(a: f64) -> f64 {
    fastrand::f64() * a
}

pub fn rand_f64() -> f64 {
    fastrand::f64()
}