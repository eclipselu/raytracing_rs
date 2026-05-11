use crate::{
    utils::{random_double, random_int_range},
    vec3::Point3,
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perlin = Self {
            randfloat: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };

        for (_, x) in perlin.randfloat.iter_mut().enumerate() {
            *x = random_double();
        }

        for i in 0..POINT_COUNT {
            perlin.randfloat[i] = random_double();
        }

        Self::gen_perm(&mut perlin.perm_x);
        Self::gen_perm(&mut perlin.perm_y);
        Self::gen_perm(&mut perlin.perm_z);

        perlin
    }

    fn gen_perm(arr: &mut [i32; POINT_COUNT]) {
        for (i, x) in arr.iter_mut().enumerate() {
            *x = i as i32;
        }

        for i in (1..POINT_COUNT).rev() {
            let target = random_int_range(0, i as i64) as usize;
            arr.swap(i, target);
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;
        let idx = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        self.randfloat[idx as usize]
    }
}
