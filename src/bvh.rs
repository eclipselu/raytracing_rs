use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::AABB,
    hittable::{Hit_Record, Hittable, Hittable_List},
    interval::Interval,
    ray::Ray,
    utils::random_int_range,
};

pub struct BVH_Node {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVH_Node {
    fn bbox_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let a_min = a.bounding_box().axis_interval(axis).min;
        let b_min = b.bounding_box().axis_interval(axis).min;

        a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
    }

    pub fn new_from_objects(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        assert!(start < end);

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        let count = end - start;
        if count <= 2 {
            left = objects[start].clone();
            right = objects[end - 1].clone();
        } else {
            let axis = random_int_range(0, 2);
            objects[start..end]
                .sort_by(|a, b| BVH_Node::bbox_compare(a.as_ref(), b.as_ref(), axis as usize));

            let mid = start + count / 2;
            left = Rc::new(BVH_Node::new_from_objects(objects, start, mid));
            right = Rc::new(BVH_Node::new_from_objects(objects, mid, end));
        }

        let bbox = AABB::new_from_bbox(left.bounding_box(), right.bounding_box());

        BVH_Node { left, right, bbox }
    }

    pub fn new(hittable_list: &mut Hittable_List) -> Self {
        let size = hittable_list.objects.len();
        BVH_Node::new_from_objects(&mut hittable_list.objects, 0, size)
    }
}

impl Hittable for BVH_Node {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit_Record> {
        if !self.bbox.hit(ray, ray_t) {
            return Option::None;
        }

        let left_result = self.left.hit(ray, ray_t);

        let mut interval = ray_t;
        if let Some(left_res) = left_result.as_ref() {
            interval.max = left_res.t;
        }

        // tightening the interval using the left result, if right hits, we get a closer hit
        // we should choose right result first
        let right_result = self.right.hit(ray, interval);
        right_result.or(left_result)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
