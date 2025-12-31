use std::rc::Rc;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

#[derive(Clone, Copy, Debug)]
pub struct Hit_Record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit_Record {
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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit_Record>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit_Record> {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return Option::None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let mut rec = Hit_Record {
            p: point,
            t: root,
            normal: normal,
            front_face: true,
        };
        rec.set_face_normal(&ray, normal);

        Option::Some(rec)
    }
}

#[derive(Default)]
pub struct Hittable_List {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable_List {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for Hittable_List {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit_Record> {
        let mut closest_so_far = ray_t.max;
        let mut rec: Option<Hit_Record> = Option::None;

        for object in self.objects.iter() {
            let tmp_rec = object.hit(
                ray,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            );
            if let Some(tmp_rec) = tmp_rec {
                rec = Option::Some(tmp_rec);
                closest_so_far = tmp_rec.t;
            }
        }

        rec
    }
}
