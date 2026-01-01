use std::{f64, rc::Rc};

use raytracing_rs::{
    camera::Camera,
    hittable::{Hittable_List, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    utils::{random_double, random_double_range},
    vec3::{Color, Point3, Vec3},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 600;
const SAMPLES_PER_PIXEL: u64 = 500;
const MAX_DEPTH: i16 = 50;

fn main() {
    // World
    let mut world = Hittable_List::new();

    let ground_material = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Rc::new(Metal { albedo, fuzz })
                } else {
                    // glass
                    Rc::new(Dielectric {
                        refraction_index: 1.5,
                    })
                };

                world.add(Rc::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    let material1: Rc<dyn Material> = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3: Rc<dyn Material> = Rc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        vfov,
        lookfrom,
        lookat,
        vup,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );
    let output_file = "out/random_scene.ppm";
    camera.render(&world, output_file).expect("render failed");
}
