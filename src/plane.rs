use crate::hittable::{Hittable, HitRecord};
use crate::vector::Vector;
use crate::ray::Ray;
use crate::color_utils::{apply_intensity};

pub struct Plane {
    pub p1: Vector, // start position
    pub p2: Vector, // endposition 1
    pub p3: Vector, // endposition 2
    pub color: u32,
}

impl Plane {
    pub fn new(p1: Vector, p2: Vector, p3: Vector, color: u32) -> Self {
        Plane { p1, p2, p3, color }
    }

    fn normal(&self) -> Vector {
        let v1 = self.p2 - self.p1; // Direction from point1 to point2
        let v2 = self.p3 - self.p1; // Direction from point1 to point3
        v1.cross(&v2).normalize() // Normal vector of the plane
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let normal = self.normal();
        let denom = ray.direction().dot(&normal);

        if denom.abs() < 1e-6 {
            return None; // Ray is parallel to the plane
        }

        let t = (self.p1 - ray.origin()).dot(&normal) / denom;

        if t < t_min || t > t_max {
            return None; // Intersection is out of bounds
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

        let denom = d00 * d11 - d01 * d01;
        let lambda = (d11 * d20 - d01 * d21) / denom;
        let mu = (d00 * d21 - d01 * d20) / denom;

        if lambda < 0.0 || mu < 0.0 || (lambda + mu) > 1.0 {
            return None; // Point is outside the triangle
        }

        Some(HitRecord {
            t,
            point: hit_point,
            normal,
        })
    }

    fn get_color_shade(&self, hit_point: Vector, light: Vector, _light_color: u32, ambient_intensity: f32) -> u32 {
        let normal = self.normal(); // Plane's normal
        let light_dir = (light - hit_point).normalize(); // Light direction

        let diffuse_intensity = normal.dot(&light_dir).max(0.0); // Diffuse component (Lambert's cosine law)
        let final_intensity = (diffuse_intensity + ambient_intensity).min(1.0); // Add ambient light

        // apply_intensity_with_color(self.color, light_color, final_intensity)
        apply_intensity(self.color, final_intensity)
    }
}
