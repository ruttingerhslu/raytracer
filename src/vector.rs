use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn draw(&self, buffer: &mut [u32], color: u32, x0: f32, y0: f32) {
        if self.x < x0 && self.y < y0 {
            let index = (self.y * x0 + self.x) as usize;
            buffer[index] = color;
        }
    }
}
