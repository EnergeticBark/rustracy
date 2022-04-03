use std::rc::Rc;
use rustracy::*;


pub fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::from(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Point3::random_in_unit_sphere();
        return 0.5 * ray_color(Ray::from(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0), 100.0)));

    let cam = Camera::new();

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &world, MAX_DEPTH);
                }
            print!("{}", write_color(&pixel_color, SAMPLES_PER_PIXEL))
        }
    }

    eprintln!("Done.");
}
