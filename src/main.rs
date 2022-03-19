use std::rc::Rc;
use rustracy::*;


pub fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
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

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0), 100.0)));

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::from(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::from(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, FOCAL_LENGTH);

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel = ray_color(r, &world);
            print!("{}", write_color(&pixel))
        }
    }

    eprintln!("Done.");
}
