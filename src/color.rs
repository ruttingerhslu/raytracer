use std::ops::{Add, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_u32(color: u32) -> Self {
        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >> 8) & 0xFF) as f32 / 255.0;
        let b = (color & 0xFF) as f32 / 255.0;
        Self { r, g, b }
    }

    pub fn to_u32(self) -> u32 {
        let r = (self.r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn apply_intensity(self, factor: f32) -> Self {
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }

    pub fn clamp(&mut self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            // r: (self.r + other.r).min(1.0),
            // g: (self.g + other.g).min(1.0),
            // b: (self.b + other.b).min(1.0),
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            r: (self.r * factor).min(1.0),
            g: (self.g * factor).min(1.0),
            b: (self.b * factor).min(1.0),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            r: (self.r * other.r).min(1.0),
            g: (self.g * other.g).min(1.0),
            b: (self.b * other.b).min(1.0),
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, divisor: f32) -> Self {
        if divisor == 0.0 {
            return self;
        }
        Self {
            r: (self.r / divisor).min(1.0),
            g: (self.g / divisor).min(1.0),
            b: (self.b / divisor).min(1.0),
        }
    }
}
