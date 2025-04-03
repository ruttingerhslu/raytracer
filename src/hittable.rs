use crate::vector::Vector;
use crate::ray::Ray;
use crate::color::Color;
use crate::light::Light;

pub struct Material {
    pub color: Color,       // Color of the material
    pub ambient_intensity: f32, // How much ambient light the material receives
    pub shininess: f32,     // Specular shininess value
}

pub struct HitRecord {
    pub point: Vector,
    pub normal: Vector,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn get_color_shade(
        &self, point_q: Vector, 
        light: &Light,
        camera_point: Vector,
    ) -> Color;

    fn get_ambient(&self) -> Color;
}
