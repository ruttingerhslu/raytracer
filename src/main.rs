use std::sync::Arc;

use raytracer::scene::Scene;
use raytracer::camera::Camera;

use raytracer::color::{Color};
use raytracer::sphere::Sphere;
use raytracer::triangle::{self};
use raytracer::vec3::{Vec3, Point3};
use raytracer::material::{Lambertian, Metal};

use raytracer::world::World;
use raytracer::light::Light;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio);

    let scene = Scene {
        camera: camera
    };

    let mut world = World::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let material_pink = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));

    world.add_hittable(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add_hittable(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add_hittable(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add_hittable(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let rotation = Vec3::new(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_4);
    for tri in triangle::cube(Point3::new(-0.3, 0.0, -0.6), 0.3, rotation, material_pink.clone()) {
        world.add_hittable(Box::new(tri));
    }

    let rotation2 = Vec3::new(0.0, std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
    for tri in triangle::cube(Point3::new(-0.1, 0.1, -0.5), 0.2, rotation2, material_pink.clone()) {
        world.add_hittable(Box::new(tri));
    }

    world.add_light(Light::new(Point3::new(-1.0, 0.0, 1.0), Color::new(1.0, 1.0, 1.0)));
    world.add_light(Light::new(Point3::new(1.0, 0.0, 0.5), Color::new(1.0, 1.0, 1.0)));

    Scene::render_scene(&scene, world, WIDTH, HEIGHT);
}

