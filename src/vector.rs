use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Vector) -> f32 {
        (*self - *other).magnitude()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Vector { x: 0.0, y: 0.0, z: 0.0 }; // Avoid division by zero
        }
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn rotate(&self, angle: f32, axis: &str) -> Vector {
        let cos_theta = angle.to_radians().cos();
        let sin_theta = angle.to_radians().sin();

        match axis {
            "x" => Vector {
                x: self.x,
                y: cos_theta * self.y - sin_theta * self.z,
                z: sin_theta * self.y + cos_theta * self.z,
            },
            "y" => Vector {
                x: cos_theta * self.x + sin_theta * self.z,
                y: self.y,
                z: -sin_theta * self.x + cos_theta * self.z,
            },
            "z" => Vector {
                x: cos_theta * self.x - sin_theta * self.y,
                y: sin_theta * self.x + cos_theta * self.y,
                z: self.z,
            },
            _ => *self,
        }
    }
}
