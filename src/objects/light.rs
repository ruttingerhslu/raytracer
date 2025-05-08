use crate::core::color::Color;
use crate::core::vec3::{Point3, Vec3};

#[derive(Clone)]
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

    pub fn from_bounds(min: Point3, max: Point3, intensity: Color) -> Self {
        let center = (min + max) * 0.5;
        let diagonal = (max - min).length();
        let offset = Vec3::new(-diagonal, diagonal, diagonal);

        let position = center + offset;

        Self {
            p: position,
            intensity: intensity
        }
    }

    pub fn position(&self) -> Point3 {
       self.p 
    }

    pub fn intensity(&self) -> Color {
       self.intensity
    }
}

