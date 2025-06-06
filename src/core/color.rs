use std::io::Write;

use crate::core::vec3::Vec3;
use crate::core::common;
 
pub type Color = Vec3;
 
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    let scale = 1.0 / samples_per_pixel as f32;
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);
 
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}

pub fn format_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    let scale = 1.0 / samples_per_pixel as f32;
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    format!(
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
}

pub fn clamp_add(color1: &Color, color2: &Color) -> Color {
    let r = (color1.x() + color2.x()).clamp(0.0, 1.0);
    let g = (color1.y() + color2.y()).clamp(0.0, 1.0);
    let b = (color1.z() + color2.z()).clamp(0.0, 1.0);
    
    Color::new(r, g, b)
}
