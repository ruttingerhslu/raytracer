use rand::Rng;

use crate::core::vec3::Vec3;

// Constants
 
pub use std::f32::consts::PI;
pub use std::f32::INFINITY;
 
// Utility functions
 
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::rng().random_range(0.0..1.0)
}
 
pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn clamp_uv(u: f32, v: f32) -> (f32, f32) {
    let u_clamped = u.clamp(0.0, 1.0);
    let v_clamped = v.clamp(0.0, 1.0);
    (u_clamped, v_clamped)
}

pub fn map_uv(u: f32, v: f32, normal: Vec3) -> (f32, f32) {
    let (x, y, z) = (normal.x(), normal.y(), normal.z());

    // Determine the dominant axis and sign
    let (dir, sign) = {
        let abs_x = x.abs();
        let abs_y = y.abs();
        let abs_z = z.abs();

        if abs_x > abs_y && abs_x > abs_z {
            (0, x.signum()) // X axis
        } else if abs_y > abs_z {
            (1, y.signum()) // Y axis
        } else {
            (2, z.signum()) // Z axis
        }
    };

    // Map direction + sign to face index (same as your original idea)
    let face_index = match (dir, sign as i32) {
        (0, 1) => 5.0,  // +X
        (0, -1) => 3.0, // -X
        (1, 1) => 2.0,  // +Y
        (1, -1) => 4.0, // -Y
        (2, 1) => 6.0,  // +Z
        (2, -1) => 1.0, // -Z
        _ => 0.0,
    };
    let u = face_index - u;
    (u, v)
}

// pub fn map_uv(u: f32, v: f32, normal: Vec3) -> (f32, f32) {
//     let (x, y, z) = (normal.x(), normal.y(), normal.z());
//
//     let (dir, sign) = {
//         let abs_x = x.abs();
//         let abs_y = y.abs();
//         let abs_z = z.abs();
//
//         if abs_x > abs_y && abs_x > abs_z {
//             (0, x.signum()) // X
//         } else if abs_y > abs_z {
//             (1, y.signum()) // Y
//         } else {
//             (2, z.signum()) // Z
//         }
//     };
//
//     match (dir, sign as i32) {
//         (0, 1) => (1.0 - u, v), // +X: flip U
//         (0, -1) => (u, v),      // -X
//         (1, 1) => (u, 1.0 - v), // +Y: flip V
//         (1, -1) => (u, v),      // -Y
//         (2, 1) => (1.0 - u, v), // +Z: flip U
//         (2, -1) => (u, v),      // -Z
//         _ => (u, v),
//     }
// }
