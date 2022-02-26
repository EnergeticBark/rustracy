use crate::vec3::*;

pub fn write_color(c: &Color) -> String {
    format!(
        "{} {} {}\n",
        (c.x() * 255.999) as i32,
        (c.y() * 255.999) as i32,
        (c.z() * 255.999) as i32
    )
}
