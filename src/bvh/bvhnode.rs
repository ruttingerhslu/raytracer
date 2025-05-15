use std::sync::Arc;
use rand::Rng;

use crate::objects::hittable::{Hittable, HitRecord};

use crate::bvh::aabb::AABB;

use crate::core::ray::Ray;

#[derive(Clone)]
pub enum BVHNode {
    Leaf {
        bounding_box: AABB,
        object: Arc<dyn Hittable>,
    },
    Internal {
        bounding_box: AABB,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

impl BVHNode {
    pub fn build(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis = rand::rng().random_range(0..3);
        objects.sort_by(|a, b| {
            let box_a = a.bounding_box().unwrap();
            let box_b = b.bounding_box().unwrap();
            box_a.min.get(axis).partial_cmp(&box_b.min.get(axis)).unwrap()
        });

        match objects.len() {
            1 => BVHNode::Leaf {
                bounding_box: objects[0].bounding_box().unwrap(),
                object: objects[0].clone(),
            },
            2 => {
                let left = BVHNode::build(vec![objects[0].clone()]);
                let right = BVHNode::build(vec![objects[1].clone()]);
                let box_left = left.bounding_box().unwrap();
                let box_right = right.bounding_box().unwrap();
                BVHNode::Internal {
                    bounding_box: AABB::surrounding_box(&box_left, &box_right),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            _ => {
                let mid = objects.len() / 2;
                let left = BVHNode::build(objects[..mid].to_vec());
                let right = BVHNode::build(objects[mid..].to_vec());
                let box_left = left.bounding_box().unwrap();
                let box_right = right.bounding_box().unwrap();
                BVHNode::Internal {
                    bounding_box: AABB::surrounding_box(&box_left, &box_right),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let bbox = match self {
            BVHNode::Leaf { bounding_box, .. } => bounding_box,
            BVHNode::Internal { bounding_box, .. } => bounding_box,
        };
    
        if !bbox.hit(ray, t_min, t_max) {
            return false;
        }
    
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::new();
        let mut closest = t_max;
    
        match self {
            BVHNode::Leaf { object, .. } => {
                if object.hit(ray, t_min, closest, &mut temp_rec) {
                    *rec = temp_rec.clone();
                    closest = temp_rec.t;
                    hit_anything = true;
                }
            }
            BVHNode::Internal { left, right, .. } => {
                if left.hit(ray, t_min, closest, &mut temp_rec) {
                    *rec = temp_rec.clone();
                    closest = temp_rec.t;
                    hit_anything = true;
                }
    
                if right.hit(ray, t_min, closest, &mut temp_rec) {
                    *rec = temp_rec.clone();
                    closest = temp_rec.t;
                    hit_anything = true;
                }
            }
        }
    
        hit_anything
    }    

    fn bounding_box(&self) -> Option<AABB> {
        Some(match self {
            BVHNode::Leaf { bounding_box, .. } => *bounding_box,
            BVHNode::Internal { bounding_box, .. } => *bounding_box,
        })
    }
}
