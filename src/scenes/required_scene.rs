use anyhow::Result;

use std::sync::Arc;
use std::path::PathBuf;

use crate::camera::Camera;
use crate::world::World;
use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use crate::material::{Lambertian, Glass, Metal, SoapBubble}; 
use crate::triangle::{self};
use crate::sphere::Sphere;
use crate::light::Light;
use crate::scenes::obj;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

pub async fn setup_scene(obj_path: &PathBuf, world: &mut World) -> Result<Camera> {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
    let metal = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

    world.add_hittable(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // let soap = Arc::new(SoapBubble::new(1.33));
    // let air = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.0));
    // world.add_hittable(Box::new(Sphere::new(
    //     Point3::new(0.7, 0.0, 1.0),
    //     0.7,
    //     soap.clone(),
    // )));
    // world.add_hittable(Box::new(Sphere::new(
    //     Point3::new(0.7, 0.0, 1.0),
    //     -0.69,
    //     air,
    // )));

    let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
    for tri in triangle::cube(Point3::new(0.0, 0.0, -2.0), 0.6, rotation, metal.clone()) {
        world.add_hittable(Box::new(tri));
    }

    let (min, max) = obj::load_obj_from_path(obj_path, world, glass).await?;

    let light_color = Color::new(0.9, 0.9, 0.9);
    let light = Light::from_bounds(min, max, light_color);
    world.add_light(light);

    let camera = Camera::from_bounds(min, max, WIDTH as f32 / HEIGHT as f32);

    Ok(camera)
}
