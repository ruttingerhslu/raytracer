#![allow(unused_imports)]
#![allow(unused_variables)]
use gltf::{self, Gltf, Semantic};
use gltf::mesh::Semantic::Positions;

use std::path::Path;
use std::sync::Arc;

use raytracer::scene::Scene;
use raytracer::camera::Camera;

use raytracer::color::{Color};
use raytracer::sphere::Sphere;
use raytracer::triangle::{self, Triangle};
use raytracer::vec3::{Vec3, Point3};
use raytracer::material::{Material, Lambertian, Metal, Glass};

use raytracer::world::World;
use raytracer::light::Light;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera::new(aspect_ratio);
    camera.set_position(Point3::new(0.0, 0.0, 0.0));
    let scene = Scene {
        camera: camera
    };

    let mut world = World::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let material_pink = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

    // let material_glass = Arc::new(Glass::new(Color::new(0.7, 0.7, 0.7), 1.5));
    // let material_glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
    let glass = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
    let air = Arc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.0));

    world.add_hittable(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        glass.clone(),
    )));

    // world.add_hittable(Box::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.45,
    //     air.clone(),
    // )));

    world.add_hittable(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add_hittable(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add_hittable(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));
    // world.add_hittable(Box::new(Sphere::new(
    //     Point3::new(0.0, 0.1, -3.0),
    //     0.5,
    //     material_right.clone(),
    // )));

    let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
    for tri in triangle::cube(Point3::new(-0.3, 0.0, -0.6), 0.3, rotation, material_pink.clone()) {
        world.add_hittable(Box::new(tri));
    }

    let rotation2 = Vec3::new(0.0, std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
    for tri in triangle::cube(Point3::new(-0.1, 0.1, -0.5), 0.2, rotation2, material_pink.clone()) {
        world.add_hittable(Box::new(tri));
    }

    let light_color = Color::new(0.5, 0.5, 0.5);
    world.add_light(Light::new(Point3::new(-1.0, 0.0, 1.0), light_color));
    world.add_light(Light::new(Point3::new(1.0, 0.0, 0.5), light_color));

    Scene::render_scene(&scene, world, WIDTH, HEIGHT);
}

