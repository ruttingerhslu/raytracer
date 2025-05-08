use crate::core::ray::Ray;

use crate::objects::hittable::{Hittable, HitRecord};
use crate::objects::light::Light;

#[derive(Clone)]
pub struct World {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            hittables: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.hittables {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

