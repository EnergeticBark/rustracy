use std::rc::Rc;
use renderer::*;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = 225;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0), 100.0)));

    let cam = Camera::from(IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let renderer = Renderer::from(IMAGE_WIDTH, IMAGE_HEIGHT);
    let bitmap = renderer.draw(&world, &cam);

    for p in bitmap {
        println!("{} {} {}", p.0, p.1, p.2)
    }

    eprintln!("Done.");
}
