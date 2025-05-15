use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::core::common;
 
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}
 
impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
 
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }
 
    pub fn x(&self) -> f32 {
        self.e[0]
    }
 
    pub fn y(&self) -> f32 {
        self.e[1]
    }
 
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn get(&self, i: usize) -> f32 {
        match i {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
 
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }
 
    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }

    pub fn normalize(&self) -> Self {
        let mag = (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt();
        if mag == 0.0 {
            return Self { e: [0.0, 0.0, 0.0] };
        }
        Self {
            e: [self.x() / mag, self.y() / mag, self.z() / mag]
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        dot(*self, *other)
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        cross(*self, other)
    }

    pub fn unit_vector(&self) -> Vec3 {
        unit_vector(*self)
    }

    pub fn max_component(&self) -> f32 {
        self.x().max(self.y()).max(self.z())
    }

    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            self.x(),
            c * self.y() - s * self.z(),
            s * self.y() + c * self.z(),
        )
    }

    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            c * self.x() + s * self.z(),
            self.y(),
            -s * self.x() + c * self.z(),
        )
    }

    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            c * self.x() - s * self.y(),
            s * self.x() + c * self.y(),
            self.z(),
        )
    }

    pub fn rotate_xyz(&self, r: Vec3) -> Vec3 {
        self.rotate_x(r.x()).rotate_y(r.y()).rotate_z(r.z())
    }
}
 
// Type alias
pub type Point3 = Vec3;
 
// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
 
// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;
 
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
 
// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
// Vec3 *= f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}
 
// Vec3 /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self = *self / t;
    }
}
 
// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}
 
// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 
// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
 
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
 
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
 
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f32) -> Option<Vec3> {
    let cos_theta = f32::min(dot(-v, n), 1.0);
    let r_out_perp = etai_over_etat * (v + cos_theta * n);
    let r_out_parallel_sq = 1.0 - r_out_perp.length_squared();

    if r_out_parallel_sq < 0.0 {
        None
    } else {
        let r_out_parallel = -r_out_parallel_sq.sqrt() * n;
        Some(r_out_perp + r_out_parallel)
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn min(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.x().min(v2.x()),  // Compare the x components
        v1.y().min(v2.y()),  // Compare the y components
        v1.z().min(v2.z())   // Compare the z components
    )
}

pub fn max(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.x().max(v2.x()),  // Compare the x components
        v1.y().max(v2.y()),  // Compare the y components
        v1.z().max(v2.z())   // Compare the z components
    )
}
