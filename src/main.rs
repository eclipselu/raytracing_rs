use std::{
    f64,
    io::{self, Write, stdout},
};

use raytracing_rs::{
    ray::Ray,
    vec3::{Color, Point3, Vec3, dot},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
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
        let rec = HitRecord {
            p: point,
            t: root,
            normal: normal,
        };

        Option::Some(rec)
    }
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 400;
const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;

fn write_color(mut w: impl Write, color: Color) -> io::Result<()> {
    let ir = (color.x * 255.999) as u64;
    let ig = (color.y * 255.999) as u64;
    let ib = (color.z * 255.999) as u64;

    writeln!(w, "{} {} {}", ir, ig, ib)
}

fn ray_color(ray: Ray) -> Color {
    let sphere = Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let rec = sphere.hit(&ray, f64::MIN, f64::MAX);
    if let Some(rec) = rec {
        let normal = (ray.at(rec.t) - sphere.center).unit_vector();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    // TODO: lerp function
    let color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    color
}

fn main() {
    // We'll also have the y-axis go up, the x-axis to the right,
    // and the negative z-axis pointing in the viewing direction.
    // (This is commonly referred to as right-handed coordinates.)

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // viewport vectors
    // horizontal
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    // vertical, y-axis pointing up
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
    let pixel_delta_v = viewport_v / (IMAGE_HEIGHT as f64);

    // locations of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    // (0,0) ┌──────────┐
    //       │    •     │ ← center
    //       └──────────┘
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - y);

        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                dir: ray_direction,
            };
            let pixel_color = ray_color(ray);
            let _ = write_color(stdout(), pixel_color);
        }
    }
    eprintln!("\rDone.                 ");
}
