use std::rc::Rc;
use crate::{Hittable, Aabb, HitRecord};

struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, r: crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        if let Some(rec) = self.left.hit(r, t_min, t_max) {
            if let Some(rec2) = self.right.hit(r, t_min, rec.t) {
                return Some(rec2);
            } else {
                return Some(rec);
            }
        }
        
        self.right.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}