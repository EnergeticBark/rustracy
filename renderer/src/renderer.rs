use crate::{Camera, Color, HitRecord, HittableList, Pixel, Ray};
use crate::{random_f64, vec3::*};

pub fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::from(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p + rec.normal + Point3::random_unit_vector();
        return 0.5 * ray_color(Ray::from(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

pub struct Renderer {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Renderer {
    pub fn from(image_width: u32, image_height: u32) -> Renderer {
        Self {
            image_width,
            image_height,
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }

    pub fn draw(&self, world: &HittableList, cam: &Camera) -> Vec<Pixel> {
        let mut bitmap: Vec<Pixel> = Vec::new();

        for j in (0..self.image_height).rev() {
            eprintln!("Scanlines remaining: {j}");
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + random_f64()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + random_f64()) / (self.image_height - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &world, self.max_depth);
                }
                bitmap.push(Pixel::from(&pixel_color, self.samples_per_pixel))
            }
        }
        bitmap
    }
}