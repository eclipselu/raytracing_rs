use crate::{
    interval::{Interval, UNIVERSE_INTERVAL},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Copy, Clone, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    pub fn new_from_extrema(a: Point3, b: Point3) -> Self {
        let x = Interval {
            min: f64::min(a.x, b.x),
            max: f64::max(a.x, b.x),
        };
        let y = Interval {
            min: f64::min(a.y, b.y),
            max: f64::max(a.y, b.y),
        };
        let z = Interval {
            min: f64::min(a.z, b.z),
            max: f64::max(a.z, b.z),
        };

        AABB { x, y, z }
    }

    pub fn new_from_bbox(a: AABB, b: AABB) -> Self {
        AABB {
            x: Interval::enclosing_interval(a.x, b.x),
            y: Interval::enclosing_interval(a.y, b.y),
            z: Interval::enclosing_interval(a.z, b.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        assert!(n < 3);

        if n == 0 {
            self.x
        } else if n == 1 {
            self.y
        } else {
            self.z
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = r.origin;
        let ray_dir = r.dir;

        for axis in 0..3 {
            let ax_intv = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let mut t0 = (ax_intv.min - ray_origin[axis]) * adinv;
            let mut t1 = (ax_intv.max - ray_origin[axis]) * adinv;

            if t0 > t1 {
                (t0, t1) = (t1, t0);
            }

            ray_t.min = f64::max(ray_t.min, t0);
            ray_t.max = f64::min(ray_t.max, t1);

            if ray_t.min >= ray_t.max {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    fn assert_interval(interval: Interval, min: f64, max: f64) {
        assert!(
            (interval.min - min).abs() < f64::EPSILON,
            "expected min {}, got {}",
            min,
            interval.min
        );
        assert!(
            (interval.max - max).abs() < f64::EPSILON,
            "expected max {}, got {}",
            max,
            interval.max
        );
    }

    #[test]
    fn new_from_extrema_orders_axes() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-1.0, 0.0, 5.0);
        let bbox = AABB::new_from_extrema(a, b);

        assert_interval(bbox.x, -1.0, 1.0);
        assert_interval(bbox.y, 0.0, 2.0);
        assert_interval(bbox.z, 3.0, 5.0);
    }

    #[test]
    fn axis_interval_returns_correct_axis() {
        let bbox = AABB::new(
            Interval {
                min: -1.0,
                max: 2.0,
            },
            Interval { min: 0.5, max: 3.5 },
            Interval {
                min: -2.0,
                max: -1.0,
            },
        );

        assert_interval(bbox.axis_interval(0), -1.0, 2.0);
        assert_interval(bbox.axis_interval(1), 0.5, 3.5);
        assert_interval(bbox.axis_interval(2), -2.0, -1.0);
    }

    #[test]
    fn hit_returns_true_for_ray_passing_through() {
        let bbox = AABB::new_from_extrema(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray {
            origin: Vec3::new(-2.0, -2.0, -2.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
            time: 0.0,
        };

        assert!(bbox.hit(&ray, UNIVERSE_INTERVAL));
    }

    #[test]
    fn hit_returns_false_for_ray_missing_box() {
        let bbox = AABB::new_from_extrema(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray {
            origin: Vec3::new(2.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
            time: 0.0,
        };

        assert!(!bbox.hit(&ray, UNIVERSE_INTERVAL));
    }
}
