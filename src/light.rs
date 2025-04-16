use crate::color::Color;
use crate::vec3::Point3;

pub struct Light {
    p: Point3,
    intensity: Color,
}

impl Light {
    pub fn new(p: Point3, intensity: Color) -> Self {
        Self {
            p: p,
            intensity: intensity,
        }
    }

    pub fn position(&self) -> Point3 {
       self.p 
    }

    pub fn intensity(&self) -> Color {
       self.intensity
    }
}

