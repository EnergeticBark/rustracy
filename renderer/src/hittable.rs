use std::rc::Rc;
use crate::material::Material;
use crate::vec3::*;
use crate::ray::*;
use crate::aabb::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from(r: Ray, outward_normal: Vec3, material: Rc<dyn Material>, t: f64) -> Self {
        // point where ray intersected with object
        let p = r.at(t);

        // check if the ray and outward normal are pointing in the same direction
        // if so, the ray is hitting the inside of the object and we need the inward normal
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}