use std::rc::Rc;

use image::{DynamicImage, GenericImageView};

use crate::{
    interval::Interval,
    vec3::{Color, Point3},
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct Solid_Color {
    pub albedo: Color,
}

impl Solid_Color {
    pub fn new(albedo: Color) -> Self {
        Solid_Color { albedo }
    }
}

impl Texture for Solid_Color {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.albedo
    }
}

pub struct Checker_Texture {
    pub inv_scale: f64,
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>,
}

impl Checker_Texture {
    pub fn new(scale: f64, even: Color, odd: Color) -> Self {
        let even_texture = Rc::new(Solid_Color::new(even));
        let odd_texture = Rc::new(Solid_Color::new(odd));

        Checker_Texture {
            inv_scale: 1.0 / scale,
            even: even_texture,
            odd: odd_texture,
        }
    }
}

impl Texture for Checker_Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i64;
        let y_int = (self.inv_scale * p.y).floor() as i64;
        let z_int = (self.inv_scale * p.z).floor() as i64;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct Image_Texture {
    pub image: Rc<DynamicImage>,
}

impl Image_Texture {
    // [low, high)
    fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x >= high {
            high - 1
        } else if x < low {
            low
        } else {
            x
        }
    }

    fn get_pixel(&self, i: u32, j: u32) -> Color {
        let (width, height) = self.image.dimensions();
        let ii = Image_Texture::clamp(i, 0, width);
        let jj = Image_Texture::clamp(j, 0, height);

        let pixel = self.image.get_pixel(ii, jj);

        let color_scale = 1.0 / 255.0;
        Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
    }
}

impl Texture for Image_Texture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let (width, height) = self.image.dimensions();
        if width == 0 || height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval { min: 0.0, max: 1.0 };
        let u = interval.clamp(u);
        let v = interval.clamp(v);

        let i = (u * width as f64) as u32;
        let j = ((1.0 - v) * height as f64) as u32;

        let pixel = self.get_pixel(i, j);
        // println!("({}, {}, {})", pixel[0], pixel[1], pixel[2]);
        pixel
    }
}
