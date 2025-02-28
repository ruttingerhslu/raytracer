use crate::{vector::Vector};

pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f32) -> Vector {
        self.origin + self.direction * t
    }
}
