use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
