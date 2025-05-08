use anyhow::{Result};
use std::sync::Arc;
use std::path::PathBuf;
use async_trait::async_trait;

use crate::io::obj;

use crate::core::camera::Camera;
use crate::core::color::Color;
use crate::core::vec3::{Point3, Vec3};

use crate::material::material::{Lambertian, Glass, Metal, TexturedMaterial}; 
use crate::material::texture::Texture;

use crate::objects::world::World;
use crate::objects::triangle::{self, Triangle};
use crate::objects::sphere::Sphere;
use crate::objects::light::Light;

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
        let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
        // let air = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.0));
        // let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

        world.add_hittable(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));

        let (min, max) = obj::load_obj_from_path(obj_path, world, glass.clone()).await?;

        // light
        let light_color = Color::new(0.9, 0.9, 0.9);
        let light = Light::from_bounds(min, max, light_color);
        world.add_light(light);

        let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
        let camera = Camera::from_bounds(min, max, aspect_ratio, angle);

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
