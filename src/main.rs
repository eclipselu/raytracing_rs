use std::{f64, rc::Rc};

use raytracing_rs::{
    camera::Camera,
    hittable::{Hittable_List, Sphere},
    vec3::Point3,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 400;

fn main() {
    // World
    let mut world = Hittable_List::new();
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Camera
    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 10, 50);
    let output_file = "out/gamme_correction.ppm";
    camera.render(&world, output_file).expect("render failed");
}
