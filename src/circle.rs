use crate::vector::Vector;

pub struct Circle {
    pub center: Vector,
    pub radius: f32,
    pub color: u32,
}

impl Circle {
    pub fn new(center: Vector, radius: f32, color: u32) -> Self {
        Self { center, radius, color }
    }

    pub fn contains(&self, point: Vector) -> bool {
        self.center.distance(&point) <= self.radius
    }

    pub fn get_color(&self) -> u32 {
        self.color
    }
}
