use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
}
 
impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, mat: Arc<dyn Material>) -> Self {
        let mut normal = vec3::cross(p2 - p1, p3 - p1);

        if vec3::dot(normal, Vec3::new(0.0, 0.0, 1.0)) < 0.0 {
            normal = -normal;
        }

        Self {
            a: p1,
            b: p2,
            c: p3,
            normal: normal,
            mat: mat
        }
    }
}
 
impl Hittable for Triangle {
    // moller trombone intersection
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let v0 = self.a;
        let v1 = self.b;
        let v2 = self.c;

        let e1 = v1 - v0;
        let e2 = v2 - v0;

        let pvec = vec3::cross(r.direction(), e2);
        let det = vec3::dot(e1, pvec);

        if det.abs() < 1e-8 {
            return false;
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin() - v0;

        let u = vec3::dot(tvec, pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = vec3::cross(tvec, e1);
        let v = vec3::dot(r.direction(), qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = vec3::dot(e2, qvec) * inv_det;
        if t < t_min || t > t_max {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.normal = self.normal;
        rec.u = u;
        rec.v = v;

        true
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
