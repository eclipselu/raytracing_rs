use std::io::{self, Write, stdout};

use crate::{
    hittable::{Hittable, Hittable_List},
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn write_color(mut w: impl Write, color: Color) -> io::Result<()> {
    let ir = (color.x * 255.999) as u64;
    let ig = (color.y * 255.999) as u64;
    let ib = (color.z * 255.999) as u64;

    writeln!(w, "{} {} {}", ir, ig, ib)
}

pub struct Camera {
    _aspect_ratio: f64,
    image_width: u64,
    image_height: u64,

    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u64) -> Self {
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
            image_width: image_width,
            image_height: image_height,

            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    fn ray_color(&self, ray: &Ray, world: &Hittable_List) -> Color {
        let rec = world.hit(
            &ray,
            Interval {
                min: 0.0,
                max: f64::MAX,
            },
        );
        if let Some(rec) = rec {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = ray.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        // TODO: lerp function
        let color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
        color
    }

    pub fn render(&self, world: &Hittable_List) {
        // Render
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - y);

            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    dir: ray_direction,
                };
                let pixel_color = self.ray_color(&ray, &world);
                let _ = write_color(stdout(), pixel_color);
            }
        }
        eprintln!("\rDone.                 ");
    }
}
