use std::sync::Arc;

use crate::core::ray::Ray;
use crate::core::vec3::{self, Point3, Vec3};

use crate::material::material::Material;

use crate::bvh::aabb::AABB;
 
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f32,
    pub front_face: bool,
    pub u: f32,
    pub v: f32,
}
 
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
 
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Option<AABB>;
}

