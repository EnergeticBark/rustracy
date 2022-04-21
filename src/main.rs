use std::rc::Rc;
use renderer::*;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = 225;

    let material_ground = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(Color::from(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::from(1.5));
    let material_right = Rc::new(Metal::from(Color::from(0.8, 0.6, 0.2), 0.0));

    let ground = Rc::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0, material_ground));
    let center = Rc::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5, material_center));
    let left = Rc::new(Sphere::from(Point3::from(-1.0, 0.0, -1.0), 0.5, material_left.clone()));
    let left_hallow = Rc::new(Sphere::from(Point3::from(-1.0, 0.0, -1.0), -0.4, material_left));
    let right = Rc::new(Sphere::from(Point3::from(1.0, 0.0, -1.0), 0.5, material_right));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(left_hallow);
    world.add(right);


    let cam = Camera::from(IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let renderer = Renderer::from(IMAGE_WIDTH, IMAGE_HEIGHT);
    let bitmap = renderer.draw(&world, &cam);

    for p in bitmap {
        println!("{} {} {}", p.0, p.1, p.2)
    }

    eprintln!("Done.");
}
