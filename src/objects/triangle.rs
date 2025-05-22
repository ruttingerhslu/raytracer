#![allow(unused)]

use std::sync::Arc;

use crate::core::ray::Ray;
use crate::core::vec3::{self, Vec3, dot, Point3};

use crate::objects::hittable::{HitRecord, Hittable};
use crate::material::material::{Material, RoomMaterials};

#[derive(Clone)]
pub struct Triangle {
    p0: Vec3, // startposition
    p1: Vec3, // endposition 1
    p2: Vec3, // endposition 2
    uv0: (f32, f32),
    uv1: (f32, f32),
    uv2: (f32, f32),
    normal: Vec3,
    mat: Arc<dyn Material>,
}
 
impl Triangle {
    pub fn new(
        p0: Vec3, p1: Vec3, p2: Vec3, 
        uv0: (f32, f32), uv1: (f32, f32), uv2: (f32, f32),
        mat: Arc<dyn Material>
    ) -> Self {
        let mut normal = vec3::cross(p1 - p0, p2 - p0);

        if dot(normal, Vec3::new(0.0, 0.0, 1.0)) < 0.0 {
            normal = -normal;
        }

        Self {
            p0: p0,
            p1: p1,
            p2: p2,
            uv0: uv0,
            uv1: uv1,
            uv2: uv2,
            normal: normal,
            mat: mat
        }
    }

    pub fn new_with_normal(
        p0: Vec3, p1: Vec3, p2: Vec3,
        uv0: (f32, f32), uv1: (f32, f32), uv2: (f32, f32),
        normal: Vec3, mat: Arc<dyn Material>
    ) -> Self {
        Self {
            p0: p0,
            p1: p1,
            p2: p2,
            uv0: uv0,
            uv1: uv1,
            uv2: uv2,
            normal: normal,
            mat: mat
        }
    }

    pub fn new_untextured(
        p0: Vec3, p1: Vec3, p2: Vec3,
        mat: Arc<dyn Material>
    ) -> Self {
        Self::new(p0, p1, p2, (0.0, 0.0), (0.0, 0.0), (0.0, 0.0), mat)
    }

    pub fn make_wall(p0: Point3, p1: Point3, p2: Point3, material: Arc<dyn Material>) -> Self {
        Triangle::new_untextured(p0, p1, p2, material)
    }

    pub fn make_quad(p0: Point3, p1: Point3, p2: Point3, p3: Point3, material: Arc<dyn Material>) -> (Self, Self) {
        (
            Triangle::new_untextured(p0, p1, p2, material.clone()),
            Triangle::new_untextured(p0, p2, p3, material),
        )
    }
}
 
impl Hittable for Triangle {
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
        let w = 1.0 - lambda - mu;
        let (u_0, v_0) = self.uv0;
        let (u_1, v_1) = self.uv1;
        let (u_2, v_2) = self.uv2;

        let u = u_0 * w + u_1 * lambda + u_2 * mu;
        let v = v_0 * w + v_1 * lambda + v_2 * mu;

        // println!("UV0: {:?}, UV1: {:?}, UV2: {:?}", self.uv0, self.uv1, self.uv2);
        rec.u = u;
        rec.v = v;
        rec.t = t;
        rec.p = hit_point;
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let normal = vec3::cross(edge1, edge2).normalize();
        rec.set_face_normal(ray, normal);
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

    tris.push(Triangle::new_untextured(p001, p101, p111, mat.clone()));
    tris.push(Triangle::new_untextured(p001, p111, p011, mat.clone()));

    tris.push(Triangle::new_untextured(p100, p000, p010, mat.clone()));
    tris.push(Triangle::new_untextured(p100, p010, p110, mat.clone()));

    tris.push(Triangle::new_untextured(p000, p001, p011, mat.clone()));
    tris.push(Triangle::new_untextured(p000, p011, p010, mat.clone()));

    tris.push(Triangle::new_untextured(p101, p100, p110, mat.clone()));
    tris.push(Triangle::new_untextured(p101, p110, p111, mat.clone()));

    tris.push(Triangle::new_untextured(p010, p011, p111, mat.clone()));
    tris.push(Triangle::new_untextured(p010, p111, p110, mat.clone()));

    tris.push(Triangle::new_untextured(p000, p100, p101, mat.clone()));
    tris.push(Triangle::new_untextured(p000, p101, p001, mat.clone()));

    tris
}

pub fn make_room_box(
    a: f32,
    vec: Vec3,
    materials: &RoomMaterials,
) -> Vec<Triangle> {
    let mut triangles = Vec::new();

    // Floor (Y = -a)
    let (f1, f2) = Triangle::make_quad(
        Point3::new(-a, -a, -a) + vec,
        Point3::new(a, -a, -a) + vec,
        Point3::new(a, -a, a) + vec,
        Point3::new(-a, -a, a) + vec,
        Arc::clone(&materials.floor),
    );

    // Ceiling (Y = +a)
    let (c1, c2) = Triangle::make_quad(
        Point3::new(-a, a, -a) + vec,
        Point3::new(a, a, -a) + vec,
        Point3::new(a, a, a) + vec,
        Point3::new(-a, a, a) + vec,
        Arc::clone(&materials.ceiling),
    );

    // Back wall (Z = -a)
    let (b1, b2) = Triangle::make_quad(
        Point3::new(-a, -a, -a) + vec,
        Point3::new(a, -a, -a) + vec,
        Point3::new(a, a, -a) + vec,
        Point3::new(-a, a, -a) + vec,
        Arc::clone(&materials.back),
    );

    // Front wall (Z = +a)
    let (fw1, fw2) = Triangle::make_quad(
        Point3::new(-a, -a, a) + vec,
        Point3::new(a, -a, a) + vec,
        Point3::new(a, a, a) + vec,
        Point3::new(-a, a, a) + vec,
        Arc::clone(&materials.front),
    );

    // Left wall (X = -a)
    let (l1, l2) = Triangle::make_quad(
        Point3::new(-a, -a, -a) + vec,
        Point3::new(-a, -a, a) + vec,
        Point3::new(-a, a, a) + vec,
        Point3::new(-a, a, -a) + vec,
        Arc::clone(&materials.left),
    );

    // Right wall (X = +a)
    let (r1, r2) = Triangle::make_quad(
        Point3::new(a, -a, -a) + vec,
        Point3::new(a, -a, a) + vec,
        Point3::new(a, a, a) + vec,
        Point3::new(a, a, -a) + vec,
        Arc::clone(&materials.right),
    );

    // triangles.extend([f1, f2, c1, c2, b1, b2, fw1, fw2, l1, l2, r1, r2]);
    triangles.extend([l1, l2, b1, b2, f1, f2]);
    triangles
}
