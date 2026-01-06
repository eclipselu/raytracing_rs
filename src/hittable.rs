use std::rc::Rc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

#[derive(Clone)]
pub struct Hit_Record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
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
    pub center: Ray,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new_static(static_center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let ray = Ray {
            origin: static_center,
            dir: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            time: 0.0,
        };

        Sphere {
            center: ray,
            radius,
            material,
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let ray = Ray {
            origin: center1,
            dir: center2 - center1,
            time: 0.0,
        };

        Sphere {
            center: ray,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit_Record> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
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
        let normal = (point - current_center) / self.radius;
        let mut rec = Hit_Record {
            p: point,
            t: root,
            normal,
            front_face: true,
            material: Rc::clone(&self.material),
        };
        rec.set_face_normal(ray, normal);

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
                closest_so_far = tmp_rec.t;
                rec = Option::Some(tmp_rec);
            }
        }

        rec
    }
}
