use rand::{thread_rng, Rng, distributions::uniform::{SampleRange, SampleUniform}};

mod vec3;
mod pixel;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod renderer;
mod material;
mod aabb;
mod bvh;

pub use vec3::*;
pub use pixel::*;
pub use ray::Ray;
pub use crate::renderer::Renderer;
pub use hittable::{Hittable, HitRecord};
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use camera::Camera;
pub use material::{Lambertian, Metal, Dielectric};
pub use aabb::Aabb;
pub use bvh::BvhNode;

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    thread_rng().gen()
}

pub fn random_in_range<T, R>(range: R) -> T
    where T: SampleUniform,
    R: SampleRange<T>
{
    thread_rng().gen_range(range)
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