use std::{f64, rc::Rc};

use raytracing_rs::{
    camera::Camera,
    hittable::{Hittable_List, Sphere},
    material::{Dielectric, Lambertian, Metal},
    vec3::{Color, Point3, Vec3},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 400;

fn main() {
    // World
    let mut world = Hittable_List::new();

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    let material_bubble = Rc::new(Dielectric {
        refraction_index: 1.0 / 1.5,
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: material_center,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.4,
        material: material_bubble,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));

    // Camera
    let lookfrom = Point3::new(-2.0, 2.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let defocus_angle = 10.0;
    let focus_dist = 4.0;

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
        10,
        50,
    );
    let output_file = "out/defocus_blur.ppm";
    camera.render(&world, output_file).expect("render failed");
}
