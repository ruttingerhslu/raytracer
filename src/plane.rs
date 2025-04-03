use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vector;
use crate::ray::Ray;
use crate::color::Color;
use crate::light::Light;

pub struct Plane {
    pub p1: Vector, // start position
    pub p2: Vector, // endposition 1
    pub p3: Vector, // endposition 2
    pub color: Color,
    pub normal: Vector,
}

impl Plane {
    pub fn new(p1: Vector, p2: Vector, p3: Vector, color: Color) -> Self {
        let mut normal = (p2 - p1).cross(&(p3 - p1)).normalize();

        if normal.dot(&Vector::new(0.0, 0.0, 1.0)) < 0.0 {
            normal = -normal;
        }

        Plane { p1, p2, p3, color, normal }
    }

    // fn normal(&self) -> Vector {
    //     let v1 = self.p2 - self.p1;
    //     let v2 = self.p3 - self.p1;
    //     v1.cross(&v2).normalize()
    // }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let normal = self.normal();
        let denom = ray.direction().dot(&self.normal);

        if denom.abs() < 1e-7 {
            return None;
        }

        let t = (self.p1 - ray.origin()).dot(&self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);

        let v0 = self.p2 - self.p1;
        let v1 = self.p3 - self.p1;
        let v2 = hit_point - self.p1;

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);

        let inv_denom = 1.0 / (d00 * d11 - d01 * d01);
        let lambda = (d11 * d20 - d01 * d21) * inv_denom;
        let mu = (d00 * d21 - d01 * d20) * inv_denom;

        if lambda < 0.0 || mu < 0.0 || (lambda + mu) > 1.0 {
            return None;
        }

        Some(HitRecord {
            t,
            point: hit_point,
            normal: self.normal,
        })
    }

    fn get_color_shade(&self, hit_point: Vector, light: &Light, camera_point: Vector) -> Color {
        let light_dir = (light.center - hit_point).normalize();
        let view_dir = (camera_point - hit_point).normalize();

        let diffuse_intensity = self.normal.dot(&light_dir).max(0.0);
        let halfway = (light_dir + view_dir).normalize();
        let specular_intensity = self.normal.dot(&halfway).max(0.0).powf(16.0); // Shininess = 16

        let diffuse = self.color * light.color * diffuse_intensity;
        let specular = light.color * specular_intensity;

        (diffuse + specular).clamp()
    }

    fn get_ambient(&self) -> Color {
        self.color * 0.5
    }
}
