use std::{f64, rc::Rc};

use raytracing_rs::{
    bvh::BVH_Node,
    camera::Camera,
    hittable::{Hittable_List, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    perlin::Perlin,
    texture::{Checker_Texture, Image_Texture, Noise_Texture},
    utils::{random_double, random_double_range},
    vec3::{Color, Point3, Vec3},
};

fn bouncing_balls_scene() {
    // World
    let mut world = Hittable_List::new();

    // Ground with checker texture
    let checker_tex: Rc<Checker_Texture> = Rc::new(Checker_Texture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_mat: Rc<Lambertian> = Rc::new(Lambertian {
        texture: checker_tex,
    });
    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, -1000.5, 0.0),
        1000.0,
        ground_mat,
    )));

    // Random small balls
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            // Move small balls away from the big balls
            if (center - Point3::new(4.0, -0.2, 0.0)).length() > 0.9 {
                let sphere: Rc<Sphere> = if choose_mat < 0.8 {
                    // Diffuse
                    let mat: Rc<Lambertian> =
                        Rc::new(Lambertian::new_solid(Color::random() * Color::random()));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    Rc::new(Sphere::new_moving(center, center2, 0.2, mat))
                } else if choose_mat < 0.95 {
                    // Metal
                    let mat: Rc<Metal> = Rc::new(Metal {
                        albedo: Color::random_range(0.5, 1.0),
                        fuzz: random_double_range(0.0, 0.5),
                    });
                    Rc::new(Sphere::new_static(center, 0.2, mat))
                } else {
                    // Glass
                    let mat: Rc<Dielectric> = Rc::new(Dielectric {
                        refraction_index: 1.5,
                    });
                    Rc::new(Sphere::new_static(center, 0.2, mat))
                };
                world.add(sphere);
            }
        }
    }

    // Three large balls
    let glass: Rc<Dielectric> = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    let brown: Rc<Lambertian> = Rc::new(Lambertian::new_solid(Color::new(0.4, 0.2, 0.1)));
    let copper: Rc<Metal> = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass,
    )));
    world.add(Rc::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        brown,
    )));
    world.add(Rc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        copper,
    )));

    let bvh = BVH_Node::new(&mut world);
    world = Hittable_List::new_from_hittable(Rc::new(bvh));

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let sample_per_pixel = 100;
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
    let output_file = "out/checker_texture.ppm";
    camera.render(&world, output_file).expect("render failed");
}

fn checked_balls_scene() {
    // World
    let mut world = Hittable_List::new();

    let checker_tex =
        Checker_Texture::new(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let mat: Rc<dyn Material> = Rc::new(Lambertian {
        texture: Rc::new(checker_tex),
    });

    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        mat.clone(),
    )));
    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        mat.clone(),
    )));

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let sample_per_pixel = 100;
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
    let output_file = "out/checked_balls.ppm";
    camera.render(&world, output_file).expect("render failed");
}

fn earth_scene() {
    let mut world = Hittable_List::new();

    let earth_img = image::open("images/earthmap.jpg").unwrap();
    let earth_texture = Image_Texture {
        image: Rc::new(earth_img),
    };

    let earth_surface: Rc<dyn Material> = Rc::new(Lambertian {
        texture: Rc::new(earth_texture),
    });

    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;

    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let sample_per_pixel = 100;
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
    let output_file = "out/earth_scene.ppm";
    camera.render(&world, output_file).expect("render failed");
}

fn perlin_spheres() {
    let mut world = Hittable_List::new();

    let perlin = Perlin::new();
    let perlin_texture = Noise_Texture {
        noise: Rc::new(perlin),
    };
    let texture: Rc<dyn Material> = Rc::new(Lambertian {
        texture: Rc::new(perlin_texture),
    });

    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        texture.clone(),
    )));
    world.add(Rc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        texture.clone(),
    )));

    // Camera
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

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
    let output_file = "out/perlin_noise.ppm";
    camera.render(&world, output_file).expect("render failed");
}

fn main() {
    // bouncing_balls_scene();
    // checked_balls_scene();
    // earth_scene();
    perlin_spheres();
}
