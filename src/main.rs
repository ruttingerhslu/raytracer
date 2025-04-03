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

    let mut cube_planes = Scene::create_cube(cube_center, 100.0, white, cube_rotation);
    cube_planes.extend(
        Scene::create_cube(Vector::new(WIDTH as f32 / 2.0 - 100.0, HEIGHT as f32 /2.0 - 20.0, -30.0), 120.0, white, Vector::new(30.0, 20.0, 60.0))
        );

    // cube_planes.extend(
    //     Scene::create_cube(Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 + 400.0, 100.0), 500.0, white, Vector::new(45.0, 20.0, 30.0))
    // );

    let vector_r = Vector::new(WIDTH as f32 / 2.0 - 50.0, HEIGHT as f32 / 2.0, 10.0);
    let vector_g = Vector::new(WIDTH as f32 / 2.0 + 50.0, HEIGHT as f32 / 2.0, 20.0);
    let vector_b = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 86.6, 40.0);

    let small_sphere = Vector::new(WIDTH as f32 / 2.0 - 60.0, HEIGHT as f32 / 2.0 -100.0, -80.0);

    let scene = Scene {
        spheres: vec![
            Sphere::new(vector_r, 100.0, Color::from_u32(0xFF0000)),
            Sphere::new(vector_g, 60.0, Color::from_u32(0x00FF00)),
            Sphere::new(vector_b, 100.0, Color::from_u32(0x0000FF)),
            Sphere::new(small_sphere, 20.0, Color::from_u32(0xFF00FF)),
        ],
        planes: cube_planes,
        lights: vec![
            Light::new(Vector::new(WIDTH as f32 / 3.0, HEIGHT as f32, -200.0), white),
            Light::new(Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -120.0), white),
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

