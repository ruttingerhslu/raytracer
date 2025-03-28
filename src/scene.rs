use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::plane::Plane;
use rayon::prelude::*;
use crate::light::Light;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub lights: Vec<Light>
}

impl Scene {

    pub fn is_occluded(&self, ray: &Ray, max_t: f32) -> bool {
        self.planes
            .iter()
            .map(|s| s as &dyn Hittable)
            .filter_map(|hittable| hittable.hit(ray, 0.001, max_t))
            .any(|hit| hit.t < max_t)
    }

    pub fn render_scene(&self, width: usize, height: usize) -> Vec<u32> {
        let mut buffer = vec![0x000000; width * height];

        buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
            let (x, y) = (index % width, index / width);
            let camera_point = Vector::new(x as f32, y as f32, -500.0);
            let direction = Vector::new(0.0, 0.0, 1.0);
            let ray = Ray::new(camera_point, direction);

            let mut color = Color::from_u32(0xFFFFFF) * 0.4;

            let mut closest_hit: Option<(&dyn Hittable, HitRecord)> = None;
            let mut closest_t = f32::INFINITY;

            for hittable in self.spheres.iter().map(|s| s as &dyn Hittable)
                .chain(self.planes.iter().map(|p| p as &dyn Hittable)) {
                if let Some(rec) = hittable.hit(&ray, 0.001, closest_t) {
                    closest_t = rec.t;
                    closest_hit = Some((hittable, rec));
                }
            }

            if let Some((hittable, rec)) = closest_hit {
                let mut total_color = Color::default();

                for light in &self.lights {
                    let shadow_origin = rec.point + rec.normal * 1e-4;
                    let shadow_ray = Ray::new(shadow_origin, (light.center - rec.point).normalize());
                    
                    total_color = total_color + hittable.get_color_shade(rec.point, light, camera_point);
                    if self.is_occluded(&shadow_ray, light.center.distance(&rec.point)) {
                        total_color = total_color * 0.5;
                    }
                }
                color = total_color.clamp();
            }
            *pixel = Color::to_u32(color);
        });

        buffer
    }

    pub fn create_cube(center: Vector, size: f32, color: Color, rotation: Vector) -> Vec<Plane> {
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
                  + center;
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


