use crate::vector::Vector;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vector,
    pub normal: Vector,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn get_color_shade(
        &self, point_q: Vector, 
        point_l: Vector, 
        light_color: u32,
        ambient_intensity: f32
    ) -> u32;
}
