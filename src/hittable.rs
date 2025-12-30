use crate::{
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = dot(ray.dir, ray.dir);
        let h = dot(ray.dir, oc);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;

        // calculate t, if no such t return a negative number
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return Option::None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let mut rec = HitRecord {
            p: point,
            t: root,
            normal: normal,
            front_face: true,
        };
        rec.set_face_normal(&ray, normal);

        Option::Some(rec)
    }
}
