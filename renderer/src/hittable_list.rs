use std::rc::Rc;
use crate::aabb::{Aabb, self};
use crate::hittable::*;
use crate::Ray;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_rec: Option<HitRecord> = None;
        let mut max_distance = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, max_distance) {
                max_distance = rec.t;
                closest_rec = Some(rec);
            }
        }

        closest_rec
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None
        }

        let mut output_box: Option<Aabb> = None;

        for object in self.objects.iter() {
            match object.bounding_box() {
                None => return None,
                Some(temp_box) => {
                    output_box = match output_box {
                        None => Some(temp_box),
                        Some(output_box) => Some(aabb::surrounding_box(output_box, temp_box)),
                    };
                },
            }
        }

        output_box
    }
}