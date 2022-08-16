use std::{rc::Rc, cmp::Ordering};
use crate::{Hittable, Aabb, HitRecord, random_in_range, aabb::surrounding_box};

struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    fn new(src_objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut left: Rc<dyn Hittable>;
        let mut right: Rc<dyn Hittable>;

        let axis = random_in_range(0..=2);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let object_span = end - start;

        match object_span {
            1 => {
                left = src_objects[start].clone();
                right = src_objects[start].clone();
            },
            2 => {
                let a = src_objects[start].clone();
                let b = src_objects[start+1].clone();
                if comparator(&a, &b) == Ordering::Less {
                    left = a;
                    right = b;
                } else {
                    left = b;
                    right = a;
                }
            },
            _ => {
                let mut sorted_objects: Vec<Rc<dyn Hittable>> =(src_objects[start..end].iter()
                    .cloned())
                    .collect();
                sorted_objects.sort_by(|a, b| comparator(a, b));

                let mid = object_span/2;

                left = Rc::new(BvhNode::new(&sorted_objects, start, mid));
                right = Rc::new(BvhNode::new(&sorted_objects, mid, end));
            }
        }

        let bounding_box = surrounding_box(left.bounding_box().unwrap(), right.bounding_box().unwrap());

        Self {
            left,
            right,
            bounding_box,
        }
    }
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

fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    a.bounding_box().unwrap().min().x().partial_cmp(&b.bounding_box().unwrap().min().x()).unwrap()
}

fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    a.bounding_box().unwrap().min().y().partial_cmp(&b.bounding_box().unwrap().min().y()).unwrap()
}

fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    a.bounding_box().unwrap().min().z().partial_cmp(&b.bounding_box().unwrap().min().z()).unwrap()
}