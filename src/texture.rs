use image::{RgbaImage};
use crate::color::{Color};

pub struct Texture {
    image: RgbaImage,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(image: RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        Self { image, width, height }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let x = (u * self.width as f32) as u32;
        let y = (v * self.height as f32) as u32;

        let pixel = self.image.get_pixel(x.min(self.width - 1), y.min(self.height - 1));
        Color::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }
}
