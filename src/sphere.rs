use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vector;
use crate::ray::Ray;
use crate::color::Color;
use crate::light::Light;

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, color: Color) -> Self {
        Self { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let h = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = -h - sqrt_d;

        if t < t_min || t > t_max {
            t = -h + sqrt_d;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        let mut normal = (point - self.center).normalize();
        let front_face = ray.direction().dot(&normal) < 0.0;
        if !front_face {
            normal = -normal;
        }

        Some(HitRecord { point, normal, t })
    }

    fn get_color_shade(
        &self, hit_point: Vector, 
        light: &Light,
        camera_point: Vector
    ) -> Color {
        let n = (hit_point - self.center).normalize();
        let s = (light.center - hit_point).normalize();
        let v = (camera_point - hit_point).normalize();

        let cos_delta = n.dot(&s).max(0.0);
        let diffuse_intensity = cos_delta;

        let halfway = (s + v).normalize();
        let specular_intensity = n.dot(&halfway).max(0.0).powf(32.0); // shininess 32.0

        let diffuse = self.color * light.color * diffuse_intensity;
        let specular = light.color * specular_intensity;

        diffuse + specular
    }

    fn get_ambient(&self) -> Color {
        self.color * 0.5
    }
}
