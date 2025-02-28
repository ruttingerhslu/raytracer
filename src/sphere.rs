use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vector;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: u32,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, color: u32) -> Self {
        Self { center, radius, color }
    }

    pub fn get_color(&self) -> u32 {
        self.color
    }

    pub fn get_color_shade(&self, point: Vector) -> u32 {
        let r_base = ((self.color >> 16) & 0xFF) as f32 / 255.0;
        let g_base = ((self.color >> 8) & 0xFF) as f32 / 255.0;
        let b_base = (self.color & 0xFF) as f32 / 255.0;

        let min_shading = 0.3;
        let max_shading = 1.0;
        let shading = min_shading + (max_shading - min_shading) * (-point.z) * 0.5;

        let r = (r_base * shading * 255.0) as u32;
        let g = (g_base * shading * 255.0) as u32;
        let b = (b_base * shading * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vector::dot(&ray.direction(), &ray.direction());
        let h = Vector::dot(&oc, &ray.direction());
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-h - sqrt_d) / a;

        if t < t_min || t > t_max {
            t = (-h + sqrt_d) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        let normal = (point - self.center).normalize();

        Some(HitRecord { point, normal, t })
    }
}
