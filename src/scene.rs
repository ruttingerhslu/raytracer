use anyhow::{Result};
use std::sync::Arc;
use std::path::PathBuf;
use async_trait::async_trait;

use crate::camera::Camera;
use crate::world::World;
use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use crate::material::{Lambertian, Glass, Metal, TexturedMaterial}; 
use crate::texture::Texture;
use crate::triangle::{self, Triangle};
use crate::sphere::Sphere;
use crate::light::Light;
use crate::obj;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[async_trait]
pub trait Scene: Send + Sync {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
    ) -> Result<Camera>;
}

pub struct CustomScene;
pub struct RequiredScene;
pub struct MuseumScene;

#[async_trait]
impl Scene for CustomScene {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
    ) -> Result<Camera> {
            let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
            let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
            let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
            let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

            // text wall
            let img = image::open("text.png")?.to_rgba8();
            let texture = Arc::new(Texture::new(img));
            let mat = Arc::new(TexturedMaterial::new(texture));
            let aspect_ratio = 3103.0 / 479.0;
            let half_width = aspect_ratio / 2.0;
            let half_height = 0.5;
            let offset_x = 0.0;
            let offset_y = 0.0;
            let offset_z = -1.5;
            let v0 = Point3::new(-half_width + offset_x, -half_height + offset_y, offset_z);
            let v1 = Point3::new( half_width + offset_x, -half_height + offset_y, offset_z);
            let v2 = Point3::new(-half_width + offset_x,  half_height + offset_y, offset_z);
            let v3 = Point3::new( half_width + offset_x,  half_height + offset_y, offset_z);
            let uv0 = (0.0, 0.0);
            let uv1 = (1.0, 0.0);
            let uv2 = (0.0, 1.0);
            let uv3 = (1.0, 1.0);
            world.add_hittable(Box::new(Triangle::new(
                v0, v1, v3,
                uv0, uv1, uv3,
                mat.clone()
            )));
            world.add_hittable(Box::new(Triangle::new(
                v0, v3, v2,
                uv0, uv3, uv2,
                mat
            )));

            // world sphere (ground)
            world.add_hittable(Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )));
            // sphere example 
            world.add_hittable(Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                material_center,
            )));

            // rotated cube
            let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
            for tri in triangle::cube(Point3::new(0.0, 0.0, -2.0), 0.6, rotation, metal.clone()) {
                world.add_hittable(Box::new(tri));
            }

            // loaded object
            let (min, max) = obj::load_obj_from_path(obj_path, world, glass).await?;

            // light
            let light_color = Color::new(0.9, 0.9, 0.9);
            let light = Light::from_bounds(min, max, light_color);
            world.add_light(light);

            let camera = Camera::from_bounds(min, max, WIDTH as f32 / HEIGHT as f32, angle);

            Ok(camera)
    }
}

#[async_trait]
impl Scene for MuseumScene {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
    ) -> Result<Camera> {
            let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));

            // Load object and get bounds
            let (min, max) = obj::load_obj_from_path(obj_path, world, glass.clone()).await?;

            // Define glass display case slightly larger than object bounds
            let padding = 0.05;
            let case_min = min - Vec3::new(padding, padding, padding);
            let case_max = max + Vec3::new(padding, padding, padding);
            let center = (case_min + case_max) / 2.0;
            let size = case_max - case_min;

            // Create glass cube around the object
            for tri in triangle::cube(center, size.x().max(size.y()).max(size.z()), Vec3::new(0.0, 0.0, 0.0), glass.clone()) {
                world.add_hittable(Box::new(tri));
            }

            // Add a light source above the case
            let light_height = case_max.y() + 0.3;
            let light_min = Point3::new(case_min.x(), light_height, case_min.z());
            let light_max = Point3::new(case_max.x(), light_height + 0.01, case_max.z());
            let light_color = Color::new(1.0, 1.0, 1.0);
            let light = Light::from_bounds(light_min, light_max, light_color);
            world.add_light(light);

            // Setup camera to frame the display case
            let camera = Camera::from_bounds(case_min, case_max, WIDTH as f32 / HEIGHT as f32, angle);

            Ok(camera)
    }
}

#[async_trait]
impl Scene for RequiredScene {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
    ) -> Result<Camera> {
            let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
            let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
            let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

            world.add_hittable(Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )));

            let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
            for tri in triangle::cube(Point3::new(0.0, 0.0, -2.0), 0.6, rotation, metal.clone()) {
                world.add_hittable(Box::new(tri));
            }

            let (min, max) = obj::load_obj_from_path(obj_path, world, glass).await?;

            let light_color = Color::new(0.9, 0.9, 0.9);
            let light = Light::from_bounds(min, max, light_color);
            world.add_light(light);

            let camera = Camera::from_bounds(min, max, WIDTH as f32 / HEIGHT as f32, angle);

            Ok(camera)
    }
}
