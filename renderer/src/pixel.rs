use crate::vec3::*;
use crate::clamp;

pub struct Pixel(pub u8, pub u8, pub u8);

impl Pixel {
    pub fn from(pixel_color: &Color, samples_per_pixel: u32) -> Self {
        let scale = 1.0 / samples_per_pixel as f64;
        let pixel_color = *pixel_color * scale;

        // Gamma correction for a gamma of 2.0
        let r = pixel_color.x().sqrt();
        let g = pixel_color.y().sqrt();
        let b = pixel_color.z().sqrt();


        Self(
            (clamp(r, 0.0, 0.999) * 256.0) as u8,
            (clamp(g, 0.0, 0.999) * 256.0) as u8,
            (clamp(b, 0.0, 0.999) * 256.0) as u8
        )
    }
}