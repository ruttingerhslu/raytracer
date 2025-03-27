use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::color_utils::apply_intensity;
use crate::plane::Plane;
use rayon::prelude::*;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
}

impl Scene {
    pub fn is_occluded(&self, ray: &Ray, max_t: f32) -> bool {
        self.spheres.iter().map(|s| s as &dyn Hittable)
            .chain(self.planes.iter().map(|p| p as &dyn Hittable)) 
            .any(|hittable| hittable.hit(ray, 0.001, max_t).is_some())
    }

    pub fn render_scene(scene: &Scene, width: usize, height: usize) -> Vec<u32> {
        let mut buffer = vec![0x000000; width * height];

        buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
            let x = (index % width) as f32;
            let y = (index / width) as f32;
            let point = Vector::new(x, y, -1000_f32);
            let direction = Vector::new(0_f32, 0_f32, 1_f32);
            let ray = Ray::new(point, direction);

            let light_color = 0xFFFFFF;
            let ambient_intensity = 0.4;
            let light = Vector::new(width as f32 / 2.0, height as f32 / 2.0, -150.0);
            let mut color = apply_intensity(light_color, ambient_intensity);

            let mut closest_t = f32::INFINITY;

            for hittable in scene.spheres.iter().map(|s| s as &dyn Hittable)
                .chain(scene.planes.iter().map(|p| p as &dyn Hittable)) 
            {
                if let Some(rec) = hittable.hit(&ray, 0.001, closest_t) {
                    closest_t = rec.t;
                    let in_shadow = {
                        let shadow_origin = rec.point + rec.normal * 1e-4;
                        let shadow_ray = Ray::new(shadow_origin, (light - rec.point).normalize());
                        scene.is_occluded(&shadow_ray, light.distance(&rec.point))
                    };
                    
                    if in_shadow {
                        color = hittable.get_color_shade(rec.point, light, light_color, ambient_intensity);
                        color = apply_intensity(color, 0.4);
                    } else {
                        color = hittable.get_color_shade(rec.point, light, light_color, ambient_intensity);
                    }
                }
            }

            *pixel = color;
        });

        buffer
    }

    pub fn create_cube(center: Vector, size: f32, color: u32, rotation: Vector) -> Vec<Plane> {
        let half = size / 2.0;

        let mut vertices = vec![
            Vector::new(-half, -half, -half),
            Vector::new(half, -half, -half),
            Vector::new(half, half, -half),
            Vector::new(-half, half, -half),
            Vector::new(-half, -half, half),
            Vector::new(half, -half, half),
            Vector::new(half, half, half),
            Vector::new(-half, half, half),
        ];

        for v in &mut vertices {
            *v = v.rotate(rotation.x, "x")
                  .rotate(rotation.y, "y")
                  .rotate(rotation.z, "z")
                  + center; // Move back to original position
        }
        vec![
            // Front face
            Plane::new(vertices[0], vertices[1], vertices[2], color),
            Plane::new(vertices[0], vertices[2], vertices[3], color),
            // Back face
            Plane::new(vertices[4], vertices[5], vertices[6], color),
            Plane::new(vertices[4], vertices[6], vertices[7], color),
            // Left face
            Plane::new(vertices[0], vertices[3], vertices[7], color),
            Plane::new(vertices[0], vertices[7], vertices[4], color),
            // Right face
            Plane::new(vertices[1], vertices[2], vertices[6], color),
            Plane::new(vertices[1], vertices[6], vertices[5], color),
            // Top face
            Plane::new(vertices[3], vertices[2], vertices[6], color),
            Plane::new(vertices[3], vertices[6], vertices[7], color),
            // Bottom face
            Plane::new(vertices[0], vertices[1], vertices[5], color),
            Plane::new(vertices[0], vertices[5], vertices[4], color),
        ]
    }

}


