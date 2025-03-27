use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vector;
use crate::ray::Ray;
use crate::color_utils::{to_rgb, from_rgb, apply_ambient, apply_diffuse};

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

}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let h = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
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

    fn get_color_shade(
        &self, point_q: Vector, 
        point_l: Vector, 
        light_color: u32,
        ambient_intensity: f32
    ) -> u32 {
        let n = (point_q - self.center).normalize();
        let s = (point_l - point_q).normalize();
        
        let cos_delta = n.dot(&s).max(0.0);

        let diffuse_intensity = 0.8;

        let ambient = apply_ambient(self.color, light_color, ambient_intensity);
        let diffuse = apply_diffuse(self.color, light_color, diffuse_intensity, cos_delta);

        let (r_a, g_a, b_a) = to_rgb(ambient);
        let (r_d, g_d, b_d) = to_rgb(diffuse);

        let r_final = (r_a + r_d).min(1.0);
        let g_final = (g_a + g_d).min(1.0);
        let b_final = (b_a + b_d).min(1.0);

        from_rgb((r_final * 255.0).clamp(0.0, 255.0) as u32, 
                 (g_final * 255.0).clamp(0.0, 255.0) as u32, 
                 (b_final * 255.0).clamp(0.0, 255.0) as u32)
    }
}
