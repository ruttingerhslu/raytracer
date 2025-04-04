use std::sync::Arc;

use raytracer::scene::Scene;
use raytracer::camera::Camera;

use raytracer::color::{Color};
use raytracer::hittable_list::HittableList;
use raytracer::sphere::Sphere;
use raytracer::vec3::{Point3};
use raytracer::material::{Lambertian, Metal};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio);

    let scene = Scene {
        camera: camera
    };

    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    Scene::render_scene(&scene, world, WIDTH, HEIGHT);
}

