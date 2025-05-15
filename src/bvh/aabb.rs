use crate::core::vec3::{self, Vec3};
use crate::core::ray::Ray;

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = vec3::min(box0.min, box1.min);
        let big = vec3::max(box0.max, box1.max);
        AABB { min: small, max: big }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction().get(i);  // Assuming get(i) method for Vec3
            let mut t0 = (self.min.get(i) - ray.origin().get(i)) * inv_d;  // Assuming get(i) for Vec3
            let mut t1 = (self.max.get(i) - ray.origin().get(i)) * inv_d;  // Assuming get(i) for Vec3

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t1 < t_max {
                t_max = t1;
            }
            if t0 > t_min {
                t_min = t0;
            }
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
