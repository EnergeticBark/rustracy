use std::rc::Rc;
use crate::hittable::*;
use crate::{Point3, Ray};
use crate::aabb::Aabb;
use crate::material::Material;
use crate::vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = vec3::dot(r.direction(), r.direction());
        let half_b = vec3::dot(r.direction(), oc);
        let c = vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None
            }
        }

        let rec = HitRecord::new(
            r,
            (r.at(root) - self.center) / self.radius,
            Rc::clone(&self.material),
            root,
        );

        Some(rec)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(if self.radius.is_sign_positive() {
            Aabb::new(
            self.center - Point3::new(self.radius, self.radius, self.radius),
            self.center + Point3::new(self.radius, self.radius, self.radius),
        )
    } else {
        Aabb::new(
            self.center + Point3::new(self.radius, self.radius, self.radius),
            self.center - Point3::new(self.radius, self.radius, self.radius),
        )
    })
    }
}