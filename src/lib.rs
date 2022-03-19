mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

pub use vec3::*;
pub use color::*;
pub use ray::Ray;
pub use hittable::{Hittable, HitRecord};
pub use hittable_list::HittableList;
pub use sphere::Sphere;

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}