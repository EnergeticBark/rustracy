use crate::vec3::*;
use crate::clamp;

pub fn write_color(pixel_color: &Color, samples_per_pixel: u32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = *pixel_color * scale;

    format!(
        "{} {} {}\n",
        (clamp(pixel_color.x(), 0.0, 0.999) * 256.0) as u32,
        (clamp(pixel_color.y(), 0.0, 0.999) * 256.0) as u32,
        (clamp(pixel_color.z(), 0.0, 0.999) * 256.0) as u32
    )
}
