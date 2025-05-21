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
use crate::objects::triangle::{self};
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
        let rotation = Vec3::ZERO;
        let translation = Vec3::ZERO;
        let (min, max) = obj::load_obj_from_path(obj_path, world, neutral, rotation, translation, 1.0).await?;

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
        _angle: f32,
        height: usize,
        width: usize,
    ) -> Result<Camera> {
        let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
        let _black = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 0.2)));
        let gold = Arc::new(Metal::new(Color::new(0.8, 0.5, 0.3), 0.8));
        let _air = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.0));


        let rotation = Vec3::ZERO;
        let translation = Vec3::new(-2.0, 2.0, -2.0);
        let (_, _) = obj::load_obj_from_path(obj_path, world, gold.clone(), rotation, translation, 10.0).await?;

        let light_color = Color::new(10.0, 10.0, 10.0);
        let light_pos = Point3::new(5.9, 6.9, -5.9);
        let light = Light::new(light_pos, light_color);
        world.add_light(light);
        let light_pos2 = Point3::new(-5.9, 2.5, -5.9);
        let light2 = Light::new(light_pos2, light_color);
        world.add_light(light2);

        let materials = RoomMaterials {
            floor: Arc::new(Lambertian::new(Color::new(0.3, 0.3, 0.3))),
            ceiling: Arc::new(Lambertian::new(Color::new(0.9, 0.9, 0.9))),
            back: Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.1))),
            front: Arc::new(Lambertian::new(Color::new(0.1, 0.8, 0.1))),
            left: Arc::new(Lambertian::new(Color::new(0.1, 0.1, 0.8))),
            right: Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.1))),
        };

        let room = triangle::make_room_box(6.0, Vec3::new(0.0, 6.0, 0.0), &materials);
        for wall in room {
            world.add_hittable(Box::new(wall));
        }

        let ground_y = 0.0; // the y-position of the ground

        let sphere_center = Point3::new(-1.0, ground_y + 1.5, 2.0); // 2.0 is the radius
        world.add_hittable(Box::new(Sphere::new(sphere_center, 1.5, glass.clone())));

        // let sphere2 = Point3::new(2.0, ground_y + 1.0, -3.0);
        // world.add_hittable(Box::new(Sphere::new(sphere2, 1.0, gold)));
        //
        // let sphere3 = Point3::new(1.0, ground_y + 1.0, 4.0);
        // world.add_hittable(Box::new(Sphere::new(sphere3, 1.0, glass.clone())));
        // world.add_hittable(Box::new(Sphere::new(sphere3, 0.9, air)));

        
        let aspect_ratio = width as f32 / height as f32;
        let lookfrom = Point3::new(5.9, 2.5, 5.9); // <-- choose position manually
        let lookat = Point3::new(0.0, 4.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0); // "up" in Y direction
        let vfov_deg = 80.0; // vertical field of view

        let camera = Camera::perspective(lookfrom, lookat, vup, vfov_deg, aspect_ratio);

        Ok(camera)
    }
}
