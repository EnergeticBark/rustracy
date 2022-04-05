use crate::vec3::*;
use crate::clamp;

pub fn write_color(pixel_color: &Color, samples_per_pixel: u32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = *pixel_color * scale;

    // Gamma correction for a gamma of 2.0
    let r = pixel_color.x().sqrt();
    let g = pixel_color.y().sqrt();
    let b = pixel_color.z().sqrt();


    format!(
        "{} {} {}\n",
        (clamp(r, 0.0, 0.999) * 256.0) as u32,
        (clamp(g, 0.0, 0.999) * 256.0) as u32,
        (clamp(b, 0.0, 0.999) * 256.0) as u32
    )
}
