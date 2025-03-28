use crate::vector::Vector;
use crate::color::Color;

pub struct Light {
    pub center: Vector,
    pub color: Color,
}

impl Light {
    pub fn new(center: Vector, color: Color) -> Self {
        Self { center, color }
    }
}
