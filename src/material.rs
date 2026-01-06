use crate::{
    hittable::Hit_Record,
    ray::Ray,
    utils::random_double,
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
    fn scatter(&self, ray_in: &Ray, rec: &Hit_Record) -> (Color, Option<Ray>) {
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
            time: ray_in.time,
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
            time: ray_in.time,
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

impl Dielectric {
    // calculates the reflectance based on Fresnel equation, approximated by Schlick's method
    fn reflectance(cos_theta: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
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

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        // total internal reflection
        let direction = if cannot_refract {
            reflect(unit_direction, rec.normal)
        } else {
            // Monte-carlo ray tracing
            // samples energy based on probability
            if Dielectric::reflectance(cos_theta, ri) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, ri)
            }
        };

        let scattered_ray = Ray {
            origin: rec.p,
            dir: direction,
            time: ray_in.time,
        };

        (attenuation, Option::Some(scattered_ray))
    }
}
