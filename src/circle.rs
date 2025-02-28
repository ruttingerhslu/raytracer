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

    pub fn intersects(&self, origin: Vector, direction: Vector) -> bool {
        let oc: Vector = self.center - origin;
        let a = Vector::dot(&direction, &direction);
        let b = -2.0_f32 * Vector::dot(&direction, &oc);
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4_f32 * a * c;

        return discriminant >= 0_f32
    }
}
