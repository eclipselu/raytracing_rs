use crate::{
    hittable::Hit_Record,
    ray::Ray,
    vec3::{Color, Vec3, dot, reflect, refract},
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
        let mut reflected = reflect(ray_in.dir, rec.normal);
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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &Hit_Record) -> (Color, Option<Ray>) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.dir.unit_vector();
        let refracted = refract(unit_direction, rec.normal, ri);

        let scattered_ray = Ray {
            origin: rec.p,
            dir: refracted,
        };

        (attenuation, Option::Some(scattered_ray))
    }
}
