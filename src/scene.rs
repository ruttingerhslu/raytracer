use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::{self, Point3};
use crate::ray::Ray;
use crate::camera::Camera;
use crate::common;

const SAMPLES_PER_PIXEL: i32 = 10;

pub struct Scene {
    pub camera: Camera,
}

impl Scene {
    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(r, 0.0, common::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render_scene(&self, width: usize, height: usize) -> Vec<u32> {
        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        let world = Arc::new(Mutex::new(world)); // Wrap in Arc<Mutex>

        let mut buffer = vec![0x000000; width * height];

        buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let (x, y) = ((index % width) as f32, (index / width) as f32);
                let (mod_x, mod_y) = (x as f32 + common::random_double(), y as f32 + common::random_double());
                let (u, v) = (mod_x / (width - 1) as f32, 1.0 - mod_y / (height - 1) as f32);
                let r = self.camera.get_ray(u, v);
                
                let world = world.lock().unwrap();
                pixel_color += Self::ray_color(&r, &*world);
            }
           
            color::write_color(pixel, pixel_color, SAMPLES_PER_PIXEL);
        });

        buffer
    }
}

