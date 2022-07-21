use std::rc::Rc;
use renderer::*;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = 225;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let ground = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    let center = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
    let left = Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()));
    let left_hallow = Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left));
    let right = Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(left_hallow);
    world.add(right);

    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;

    let cam = Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let renderer = Renderer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let bitmap = renderer.draw(&world, &cam);

    for p in bitmap {
        println!("{} {} {}", p.0, p.1, p.2)
    }

    eprintln!("Done.");
}
