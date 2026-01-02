use std::{f64, rc::Rc};

use raytracing_rs::{
    camera::Camera,
    hittable::{Hittable_List, Sphere},
    material::{Dielectric, Lambertian, Metal},
    utils::{random_double, random_double_range},
    vec3::{Color, Point3, Vec3},
};

fn main() {
    // World
    let mut world = Hittable_List::new();

    let ground_mat = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, -1000.5, 0.0),
        radius: 1000.0,
        material: ground_mat,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            // move small balls away from the big balls
            if (center - Point3::new(4.0, -0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let material = Rc::new(Lambertian {
                        albedo: Color::random() * Color::random(),
                    });
                    world.add(Rc::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: material,
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let material = Rc::new(Metal {
                        albedo: Color::random_range(0.5, 1.0),
                        fuzz: random_double_range(0.0, 0.5),
                    });
                    world.add(Rc::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: material,
                    }));
                } else {
                    // glass
                    let material = Rc::new(Dielectric {
                        refraction_index: 1.5,
                    });
                    world.add(Rc::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: material,
                    }));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));
    let material2 = Rc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));
    let material3 = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let sample_per_pixel = 10;
    let max_depth = 50;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
        sample_per_pixel,
        max_depth,
    );
    let output_file = "out/final_scene_small.ppm";
    camera.render(&world, output_file).expect("render failed");
}
