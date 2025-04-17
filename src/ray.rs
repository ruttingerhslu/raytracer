use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    pub current_ior: f32, // index of retraction
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            current_ior: 1.0,
        }
    }

    pub fn with_ior(origin: Point3, direction: Vec3, ior: f32) -> Self {
        Ray {
            origin,
            direction,
            current_ior: ior,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}
