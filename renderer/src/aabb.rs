use crate::{Point3, Ray};

#[derive(Clone, Copy)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    fn min(&self) -> Point3 {
        self.minimum
    }

    fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self {
            minimum,
            maximum
        }
    }

    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        // The farthest t0 needs to be closer than the closest t1, otherwise we didn't hit the box.
        // Checking x, y, and z by hand instead of looping, because apparently you can't
        // dynamically index tuples...
        // todo: Rewrite this function to be less ugly and more efficient.
        let t0 = ((self.minimum.x() - r.origin().x()) / r.direction().x())
            .min((self.maximum.x() - r.origin().x()) / r.direction().x());
        let t1 = ((self.minimum.x() - r.origin().x()) / r.direction().x())
            .max((self.maximum.x() - r.origin().x()) / r.direction().x());
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);
        if t_max <= t_min {
            return false;
        }

        let t0 = ((self.minimum.y() - r.origin().y()) / r.direction().y())
            .min((self.maximum.y() - r.origin().y()) / r.direction().y());
        let t1 = ((self.minimum.y() - r.origin().y()) / r.direction().y())
            .max((self.maximum.y() - r.origin().y()) / r.direction().y());
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);
        if t_max <= t_min {
            return false;
        }

        let t0 = ((self.minimum.z() - r.origin().z()) / r.direction().z())
            .min((self.maximum.z() - r.origin().z()) / r.direction().z());
        let t1 = ((self.minimum.z() - r.origin().z()) / r.direction().z())
            .max((self.maximum.z() - r.origin().z()) / r.direction().z());
        t_min = t0.max(t_min);
        t_max = t1.min(t_max);
        if t_max <= t_min {
            return false;
        }

        true
    }
}

pub fn surrounding_box(box_0: Aabb, box_1: Aabb) -> Aabb {
    Aabb {
        minimum: Point3::new(
            box_0.minimum.x().min(box_1.minimum.x()),
            box_0.minimum.y().min(box_1.minimum.y()),
            box_0.minimum.z().min(box_1.minimum.z()),
        ),
        maximum: Point3::new(
            box_0.maximum.x().max(box_1.maximum.x()),
            box_0.maximum.y().max(box_1.maximum.y()),
            box_0.maximum.z().max(box_1.maximum.z()),
        ),
    }
}