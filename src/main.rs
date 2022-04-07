use std::rc::Rc;
use renderer::*;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0), 100.0)));

    let cam = Camera::new();

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let renderer = Renderer::default();
    let bitmap = renderer.draw(&world, &cam);

    for p in bitmap {
        println!("{} {} {}", p.0, p.1, p.2)
    }

    eprintln!("Done.");
}
