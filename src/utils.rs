use std::f64::consts::PI;

use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn linear_to_gamma(val: f64) -> f64 {
    if val > 0.0 { val.sqrt() } else { 0.0 }
}

pub fn degrees_to_radian(d: f64) -> f64 {
    d / 180.0 * PI
}
