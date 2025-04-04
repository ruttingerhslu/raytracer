use minifb::{Key, Window, WindowOptions};

use raytracer::scene::Scene;
use raytracer::camera::Camera;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio);

    let scene = Scene {
        camera: camera
    };

    let mut window = Window::new("Scene", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap();

    let frame = Scene::render_scene(&scene, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}

