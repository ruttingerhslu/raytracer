// use toml;
// use std::fs;
// use serde::Deserialize;
//
// use minifb::{Key, Window, WindowOptions};
//
// use raytracer::vector::Vector;
// use raytracer::sphere::Sphere;
// use raytracer::scene::Scene;
// use raytracer::light::Light;
// use raytracer::color::Color;
//
// #[derive(Deserialize)]
// struct SceneConfig {
//     scene: SceneSettings,
//     cube: CubeSettings,
//     spheres: Vec<SphereSettings>,
//     lights: Vec<LightSettings>,
// }
//
// #[derive(Deserialize)]
// struct SceneSettings {
//     width: u32,
//     height: u32,
// }
//
// #[derive(Deserialize)]
// struct CubeSettings {
//     center: VectorSettings,
//     rotation: VectorSettings,
//     size: f32,
//     color: u32,
// }
//
// #[derive(Deserialize)]
// struct SphereSettings {
//     position: VectorSettings,
//     radius: f32,
//     color: u32,
// }
//
// #[derive(Deserialize)]
// struct LightSettings {
//     position: VectorSettings,
//     color: u32,
// }
//
// #[derive(Deserialize)]
// struct VectorSettings {
//     x: f32,
//     y: f32,
//     z: f32,
// }
//
// fn main() {
//     let config_data = fs::read_to_string("scene_config.toml").unwrap();
//     let config: SceneConfig = toml::de::from_str(&config_data).unwrap();
//
//     println!("Cube color: {}", config.cube.color);
//
//     let white = Color::from_u32(config.cube.color);
//     let cube_center = Vector::new(config.cube.center.x, config.cube.center.y, config.cube.center.z);
//     let cube_rotation = Vector::new(config.cube.rotation.x, config.cube.rotation.y, config.cube.rotation.z);
//
//     let cube_planes = Scene::create_cube(cube_center, config.cube.size, white, cube_rotation);
//
//     let spheres: Vec<Sphere> = config.spheres.iter().map(|sphere| {
//         Sphere::new(
//             Vector::new(sphere.position.x, sphere.position.y, sphere.position.z),
//             sphere.radius,
//             Color::from_u32(sphere.color),
//         )
//     }).collect();
//
//     let lights: Vec<Light> = config.lights.iter().map(|light| {
//         Light::new(
//             Vector::new(light.position.x, light.position.y, light.position.z),
//             Color::from_u32(light.color),
//         )
//     }).collect();
//
//     let scene = Scene {
//         spheres,
//         planes: cube_planes,
//         lights,
//     };
//
//     let mut window = Window::new("Scene", config.scene.width as usize, config.scene.height as usize, WindowOptions::default()).unwrap();
//
//     let frame = Scene::render_scene(&scene, config.scene.width as usize, config.scene.height as usize);
//
//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         window.update_with_buffer(&frame, config.scene.width as usize, config.scene.height as usize).unwrap();
//     }
// }
//
use minifb::{Key, Window, WindowOptions};

use raytracer::vector::Vector;
use raytracer::sphere::Sphere;
use raytracer::scene::Scene;
use raytracer::light::Light;
use raytracer::color::Color;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let white = Color::from_u32(0xFFFFFF);
    let cube_center = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -40.0);
    let cube_rotation = Vector::new(30.0, 45.0, 0.0);

    let cube_planes = Scene::create_cube(cube_center, 100.0, white, cube_rotation);

    let vector_r = Vector::new(WIDTH as f32 / 2.0 - 50.0, HEIGHT as f32 / 2.0, 10.0);
    let vector_g = Vector::new(WIDTH as f32 / 2.0 + 50.0, HEIGHT as f32 / 2.0, 20.0);
    let vector_b = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 86.6, 40.0);

    let scene = Scene {
        spheres: vec![
            Sphere::new(vector_r, 100.0, Color::from_u32(0xFF0000)),
            Sphere::new(vector_g, 60.0, Color::from_u32(0x00FF00)),
            Sphere::new(vector_b, 100.0, Color::from_u32(0x0000FF)),
        ],
        planes: cube_planes,
        lights: vec![
            Light::new(Vector::new(WIDTH as f32 / 3.0, HEIGHT as f32, -200.0), white),
            Light::new(Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -150.0), white),
            Light::new(Vector::new(0.0, 0.0, 0.0), white),
        ],
    };

    let mut window = Window::new("Scene", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap();

    let frame = Scene::render_scene(&scene, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}

