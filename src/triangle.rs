use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Vec3, Point3, dot};
use crate::material::Material;

#[derive(Clone)]
pub struct Triangle {
    p0: Vec3, // startposition
    p1: Vec3, // endposition 1
    p2: Vec3, // endposition 2
    normal: Vec3,
    mat: Arc<dyn Material>,
}
 
impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, mat: Arc<dyn Material>) -> Self {
        let mut normal = vec3::cross(p1 - p0, p2 - p0);

        if dot(normal, Vec3::new(0.0, 0.0, 1.0)) < 0.0 {
            normal = -normal;
        }

        Self {
            p0: p0,
            p1: p1,
            p2: p2,
            normal: normal,
            mat: mat
        }
    }
}
 
impl Hittable for Triangle {
    // // moller trombone intersection
    // fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
    //     let v0 = self.p0;
    //     let v1 = self.p1;
    //     let v2 = self.p2;
    //
    //     let e1 = v1 - v0;
    //     let e2 = v2 - v0;
    //
    //     let pvec = vec3::cross(r.direction(), e2);
    //     let det = vec3::dot(e1, pvec);
    //
    //     if det.abs() < 1e-8 {
    //         return false;
    //     }
    //
    //     let inv_det = 1.0 / det;
    //     let tvec = r.origin() - v0;
    //
    //     let u = vec3::dot(tvec, pvec) * inv_det;
    //     if u < 0.0 || u > 1.0 {
    //         return false;
    //     }
    //
    //     let qvec = vec3::cross(tvec, e1);
    //     let v = vec3::dot(r.direction(), qvec) * inv_det;
    //     if v < 0.0 || u + v > 1.0 {
    //         return false;
    //     }
    //
    //     let t = vec3::dot(e2, qvec) * inv_det;
    //     if t < t_min || t > t_max {
    //         return false;
    //     }
    //
    //     rec.t = t;
    //     rec.p = r.at(t);
    //     rec.normal = self.normal;
    //     rec.mat = Some(self.mat.clone());
    //     rec.u = u;
    //     rec.v = v;
    //     rec.set_face_normal(r, self.normal.normalize());
    //
    //     true
    // }

    // barycentric method
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let denom = dot(ray.direction(), self.normal);
        if denom.abs() < 1e-3 {
            return false;
        }

        let t = dot(self.p0 - ray.origin(), self.normal) / denom;
        if t < t_min || t > t_max {
            return false;
        }

        let hit_point = ray.at(t);

        let v0 = self.p1 - self.p0;
        let v1 = self.p2 - self.p0;
        let v2 = hit_point - self.p0;

        let d00 = dot(v0, v0);
        let d01 = dot(v0, v1);
        let d11 = dot(v1, v1);
        let d20 = dot(v2, v0);
        let d21 = dot(v2, v1);

        let inv_denom = 1.0 / (d00 * d11 - d01 * d01);
        let lambda = (d11 * d20 - d01 * d21) * inv_denom;
        let mu = (d00 * d21 - d01 * d20) * inv_denom;

        if lambda < 0.0 || mu < 0.0 || (lambda + mu) > 1.0 {
            return false;
        }

        rec.t = t;
        rec.p = hit_point;
        rec.normal = self.normal;
        rec.set_face_normal(ray, self.normal.normalize());
        rec.mat = Some(self.mat.clone());
        true
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

pub fn cube(center: Point3, size: f32, rotation: Vec3, mat: Arc<dyn Material>) -> Vec<Triangle> {
    let hs = size / 2.0;

    let mut points = [
        Vec3::new(-hs, -hs, -hs),
        Vec3::new(-hs, -hs,  hs),
        Vec3::new(-hs,  hs, -hs),
        Vec3::new(-hs,  hs,  hs),
        Vec3::new( hs, -hs, -hs),
        Vec3::new( hs, -hs,  hs),
        Vec3::new( hs,  hs, -hs),
        Vec3::new( hs,  hs,  hs),
    ];

    fn rotate_x(p: Vec3, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            p.x(),
            c * p.y() - s * p.z(),
            s * p.y() + c * p.z(),
        )
    }

    fn rotate_y(p: Vec3, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            c * p.x() + s * p.z(),
            p.y(),
            -s * p.x() + c * p.z(),
        )
    }

    fn rotate_z(p: Vec3, angle: f32) -> Vec3 {
        let (s, c) = angle.sin_cos();
        Vec3::new(
            c * p.x() - s * p.y(),
            s * p.x() + c * p.y(),
            p.z(),
        )
    }

    fn rotate_xyz(p: Vec3, r: Vec3) -> Vec3 {
        let p = rotate_x(p, r.x());
        let p = rotate_y(p, r.y());
        rotate_z(p, r.z())
    }

    for p in &mut points {
        *p = rotate_xyz(*p, rotation) + center;
    }

    let [p000, p001, p010, p011, p100, p101, p110, p111] = points;

    let mut tris = vec![];

    tris.push(Triangle::new(p001, p101, p111, mat.clone()));
    tris.push(Triangle::new(p001, p111, p011, mat.clone()));

    tris.push(Triangle::new(p100, p000, p010, mat.clone()));
    tris.push(Triangle::new(p100, p010, p110, mat.clone()));

    tris.push(Triangle::new(p000, p001, p011, mat.clone()));
    tris.push(Triangle::new(p000, p011, p010, mat.clone()));

    tris.push(Triangle::new(p101, p100, p110, mat.clone()));
    tris.push(Triangle::new(p101, p110, p111, mat.clone()));

    tris.push(Triangle::new(p010, p011, p111, mat.clone()));
    tris.push(Triangle::new(p010, p111, p110, mat.clone()));

    tris.push(Triangle::new(p000, p100, p101, mat.clone()));
    tris.push(Triangle::new(p000, p101, p001, mat.clone()));

    tris
}
