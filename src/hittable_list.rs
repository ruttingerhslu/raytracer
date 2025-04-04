use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
 
#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
 
impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
 
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
 
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
 
        hit_anything
    }
    
    fn box_clone(&self) -> Box<dyn Hittable> {
        let cloned_objects = self.objects.iter().map(|obj| obj.box_clone()).collect::<Vec<Box<dyn Hittable>>>();
        Box::new(HittableList { objects: cloned_objects })
    }
}

impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        (**self).hit(ray, t_min, t_max, rec)
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        self.as_ref().box_clone()
    }
}
