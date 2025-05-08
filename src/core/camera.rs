use std::fmt::{Display, Formatter, Result};

use crate::core::vec3::{Point3, Vec3};
use crate::core::ray::Ray;

#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn perspective(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov_deg: f32, aspect_ratio: f32) -> Self {
        let theta = vfov_deg.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn from_bounds(min: Point3, max: Point3, aspect_ratio: f32, angle: f32) -> Self {
        let center = (min + max) * 0.5;
        let diagonal = (max - min).length();
        let radius = diagonal * 1.5;

        let x = radius * angle.cos();
        let z = radius * angle.sin();
        let y = diagonal * 0.5;

        let lookfrom = center + Vec3::new(x, y, z);
        let lookat = center;
        let vup = Vec3::new(0.0, 1.0, 0.0);

        Self::perspective(lookfrom, lookat, vup, 45.0, aspect_ratio)
    }

    pub fn set_position(&mut self, position: Point3) {
        self.origin = position;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn from_gltf(position: Vec3, direction: Vec3, up: Vec3, fov_degrees: f32, aspect_ratio: f32) -> Self {
        let theta = fov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = direction.normalize() * -1.0;
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let origin = position;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }         
    }

    pub fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        
        Camera::new(aspect_ratio)
    }
}


impl Display for Camera {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.origin)
    }
}
