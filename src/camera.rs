use std::{
    f64,
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::{
    hittable::{Hittable, Hittable_List},
    interval::Interval,
    ray::Ray,
    utils::{linear_to_gamma, random_double},
    vec3::{Color, Point3, Vec3},
};

fn write_color(mut w: impl Write, color: Color) -> io::Result<()> {
    let intentsity = Interval {
        min: 0.0,
        max: 0.999,
    };

    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);
    let ir = (intentsity.clamp(r) * 256.0) as u64;
    let ig = (intentsity.clamp(g) * 256.0) as u64;
    let ib = (intentsity.clamp(b) * 256.0) as u64;

    writeln!(w, "{} {} {}", ir, ig, ib)
}

pub struct Camera {
    _aspect_ratio: f64,
    image_width: u64,
    image_height: u64,

    sample_per_pixel: u8,

    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below

    max_depth: i16,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u64, sample_per_pixel: u8, max_depth: i16) -> Self {
        let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
        // We'll also have the y-axis go up, the x-axis to the right,
        // and the negative z-axis pointing in the viewing direction.
        // (This is commonly referred to as right-handed coordinates.)

        // Camera
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // viewport vectors
        // horizontal
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        // vertical, y-axis pointing up
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // locations of the upper left pixel
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        // (0,0) ┌──────────┐
        //       │    •     │ ← center
        //       └──────────┘
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            _aspect_ratio: aspect_ratio,
            image_width,
            image_height,

            sample_per_pixel,

            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth,
        }
    }

    fn ray_color(&self, ray: &Ray, world: &Hittable_List, depth: i16) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let rec = world.hit(
            ray,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
        );

        if let Some(rec) = rec {
            let (attenuation, scattered_ray) = rec.material.scatter(ray, &rec);
            if let Some(scattered_ray) = scattered_ray {
                return attenuation * self.ray_color(&scattered_ray, world, depth - 1);
            }
            return attenuation; // default (0, 0, 0)
            // return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        // TODO: lerp function
        // Background color
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    // Construct a camera ray originating from the origin and directed at randomly sampled
    // point around the pixel location (x, y)
    fn get_ray(&self, x: u64, y: u64) -> Ray {
        let offset = Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0);
        let pixel_center = self.pixel00_loc
            + (x as f64 + offset.x) * self.pixel_delta_u
            + (y as f64 + offset.y) * self.pixel_delta_v;
        let ray_direction = pixel_center - self.center;

        Ray {
            origin: self.center,
            dir: ray_direction,
        }
    }

    pub fn render(&self, world: &Hittable_List, file_name: &str) -> io::Result<()> {
        let file = File::create(file_name)?;
        let mut out = BufWriter::new(file);

        // Render
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;

        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - y);

            for x in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, world, self.max_depth);
                }
                pixel_color *= pixel_samples_scale;
                write_color(&mut out, pixel_color)?;
            }
        }
        eprintln!("\rDone.                 ");
        Ok(())
    }
}
