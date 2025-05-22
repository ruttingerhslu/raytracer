#![allow(unused)]

use anyhow::{Result};
use std::sync::Arc;
use std::path::PathBuf;
use async_trait::async_trait;

use crate::io::obj;

use crate::core::camera::Camera;
use crate::core::color::Color;
use crate::core::vec3::{Point3, Vec3};

use crate::material::material::{Lambertian, Glass, Metal, RoomMaterials}; 

use crate::objects::world::World;
use crate::objects::triangle::{self, Triangle};
use crate::objects::sphere::Sphere;
use crate::objects::light::Light;

#[async_trait]
pub trait Scene: Send + Sync {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
        height: usize,
        width: usize,
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
        height: usize,
        width: usize,
    ) -> Result<Camera> {
        let neutral = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
        let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
        let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));
        let gold = Arc::new(Metal::new(Color::new(0.8, 0.5, 0.3), 0.8));

        // world sphere (ground)
        world.add_hittable(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));
        // sphere example 
        world.add_hittable(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            0.5,
            material_center,
        )));

        // rotated cube
        let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
        for tri in triangle::cube(Point3::new(-1.0, 0.0, 1.0), 1.0, rotation, metal.clone()) {
            world.add_hittable(Box::new(tri));
        }

        // loaded object
        let rotation = Vec3::new(std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_3, 0.0);
        let translation = Vec3::ZERO;
        let (min, max) = obj::load_obj_from_path(obj_path, world, gold, rotation, translation, 5.0).await?;

        // light
        let light_color = Color::new(10.0, 10.0, 10.0);
        let light = Light::from_bounds(min, max, light_color);
        world.add_light(light);

        let camera = Camera::from_bounds(min, max, width as f32 / height as f32, angle, 0.5);

        Ok(camera)
    }
}

#[async_trait]
impl Scene for MuseumScene {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        _angle: f32,
        height: usize,
        width: usize,
    ) -> Result<Camera> {
        let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
        // let air = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.0));
        // let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

        world.add_hittable(Box::new(Sphere::new(
            Point3::new(0.0, -105.0, -1.0),
            100.0,
            material_ground,
        )));

        let rotation = Vec3::ZERO;
        let translation = Vec3::ZERO;
        let (min, max) = obj::load_obj_from_path(obj_path, world, glass.clone(), rotation, translation, 4.0).await?;

        // light
        let light_color = Color::new(0.9, 0.9, 0.9);
        let light = Light::from_bounds(min, max, light_color);
        world.add_light(light);

        let aspect_ratio = width as f32 / height as f32;
        let camera = Camera::from_bounds(min, max, aspect_ratio, 120.0, 0.5);

        Ok(camera)
    }
}

// presentation 5-10 minutes
// sphere in a corner that is transparent -> text is shown through sphere
//
// show unique feature of program
#[async_trait]
impl Scene for RequiredScene {
    async fn setup(
        &self,
        obj_path: &PathBuf,
        world: &mut World,
        angle: f32,
        height: usize,
        width: usize,
    ) -> Result<Camera> {
        let glass = Arc::new(Glass::new(Color::new(0.8, 0.9, 1.0), 1.5));
        let red = Arc::new(Lambertian::new(Color::new(0.9, 0.1, 0.1)));
        let green = Arc::new(Lambertian::new(Color::new(0.1, 0.9, 0.1)));
        let blue = Arc::new(Lambertian::new(Color::new(0.1, 0.1, 0.9)));
        let yellow = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 0.2)));

        // Kugel zentriert bei (0, 1, 0) mit Radius 1.5
        let sphere_center = Point3::new(0.0, 2.0, 0.0);
        world.add_hittable(Box::new(Sphere::new(sphere_center, 1.5, glass.clone())));

        // Ebene als großes Dreieck (einfaches Dreieck in XZ Ebene, y=0)
        let p0 = Point3::new(-5.0, 0.0, -5.0);
        let p1 = Point3::new(5.0, 0.0, -5.0);
        let p2 = Point3::new(0.0, 0.0, 5.0);
        world.add_hittable(Box::new(Triangle::new_untextured(p0, p1, p2, green.clone())));

        // Lichtquelle
        let light_pos = Point3::new(4.0, 5.0, 4.0);
        let light_color = Color::new(15.0, 15.0, 15.0);
        let light = Light::new(light_pos, light_color);
        world.add_light(light);

        // Kamera
        let radius = 3.0;

        let min = sphere_center - Vec3::new(radius, radius, radius);
        let max = sphere_center + Vec3::new(radius, radius, radius);

        let aspect_ratio = width as f32 / height as f32;
        let height_offset_factor = 1.0;

        let camera = Camera::from_bounds(min, max, aspect_ratio, angle, height_offset_factor);

        Ok(camera)
    }
}
