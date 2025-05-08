use std::sync::Arc;

use crate::core::ray::Ray;
use crate::core::vec3::{self, Point3};

use crate::objects::hittable::{HitRecord, Hittable};
use crate::material::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<dyn Material>,
}
 
impl Sphere {
    pub fn new(cen: Point3, r: f32, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: mat
        }
    }
}
 
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
 
        let sqrt_d = f32::sqrt(discriminant);
 
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
 
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
