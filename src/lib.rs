use rand::{thread_rng, Rng};

mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

pub use vec3::*;
pub use color::*;
pub use ray::Ray;
pub use hittable::{Hittable, HitRecord};
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use camera::Camera;

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    thread_rng().gen()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}