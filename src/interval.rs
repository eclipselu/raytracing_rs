use std::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn size(self) -> f64 {
        self.max - self.min
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};
pub const EMPTY_INTERVAL: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
