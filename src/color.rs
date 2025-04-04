use crate::vec3::Vec3;
 
pub type Color = Vec3;
 
pub fn write_color(pixel: &mut u32, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    *pixel = 
        ((255.999 * r.clamp(0.0, 1.0)) as u32) << 16 |
        ((255.999 * g.clamp(0.0, 1.0)) as u32) << 8 |
        ((255.999 * b.clamp(0.0, 1.0)) as u32);
}
