use std::rc::Rc;

use crate::vec3::{Color, Point3};

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
