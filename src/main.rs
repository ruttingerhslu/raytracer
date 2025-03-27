use minifb::{Key, Window, WindowOptions};
use raytracer::vector::Vector;
use raytracer::sphere::Sphere;
use raytracer::scene::Scene;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let cube_center = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -40.0);
    let cube_size = 100.0;
    let cube_color = 0xFFFFFF; // White cube
    let cube_rotation = Vector::new(30.0, 45.0, 0.0); // Rotate 30° around X, 45° around Y

    let cube_planes = Scene::create_cube(cube_center, cube_size, cube_color, cube_rotation);

    let vector_r = Vector::new(WIDTH as f32 / 2.0 - 50.0, HEIGHT as f32 / 2.0, 10.0);
    let vector_g = Vector::new(WIDTH as f32 / 2.0 + 50.0, HEIGHT as f32 / 2.0, 20.0);
    let vector_b = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 86.6, 40.0);

    let scene = Scene {
        spheres: vec![
            Sphere::new(vector_r, 100.0, 0xFF0000), // Red
            Sphere::new(vector_g, 60.0, 0x00FF00),  // Green
            Sphere::new(vector_b, 100.0, 0x0000FF), // Blue
        ],
        planes: cube_planes,
    };

    let mut window = Window::new("Scene", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap();

    let frame = Scene::render_scene(&scene, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}
