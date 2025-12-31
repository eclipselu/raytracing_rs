use crate::{
    hittable::Hit_Record,
    ray::Ray,
    vec3::{Color, Vec3, dot},
};

pub trait Material {
    // returns: (attenuation, scattered ray)
    fn scatter(&self, _ray_in: &Ray, _rec: &Hit_Record) -> (Color, Option<Ray>) {
        (Color::new(0.0, 0.0, 0.0), Option::None)
    }
}

pub struct Lambertian {
    // Whiteness of the diffused ray
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &Hit_Record) -> (Color, Option<Ray>) {
        // NOTE: this also generates a direction on the same hemisphere as the normal
        // but the distribution changed, the added normal vector shifts the distribution towards the normal,
        // it is no longer the uniform distribution.
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered_ray = Ray {
            origin: rec.p,
            dir: scatter_direction,
        };

        (self.albedo, Option::Some(scattered_ray))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &Hit_Record) -> (Color, Option<Ray>) {
        let mut reflected = ray_in.dir.reflect(rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let scattered_ray = Ray {
            origin: rec.p,
            dir: reflected,
        };

        if dot(scattered_ray.dir, rec.normal) > 0.0 {
            (self.albedo, Option::Some(scattered_ray))
        } else {
            (self.albedo, Option::None)
        }
    }
}
