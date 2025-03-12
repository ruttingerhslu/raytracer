use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

use raytracer::vector::Vector;
use raytracer::sphere::Sphere;
use raytracer::ray::Ray;
use raytracer::hittable::Hittable;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Scene {
    spheres: Vec<Sphere>,
}

fn render_scene(scene: &Scene) -> Vec<u32> {
    let mut buffer = vec![0x000000; WIDTH * HEIGHT];

    buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
        let x = (index % WIDTH) as f32;
        let y = (index / WIDTH) as f32;
        let point = Vector::new(x, y, -100_f32);
        let direction = Vector::new(0_f32, 0_f32, 1_f32);
        let ray = Ray::new(point, direction);

        let mut color = 0x000000;

        let mut closest_t = f32::INFINITY;

        for sphere in &scene.spheres {
            color = if let Some(rec) = sphere.hit(&ray, 0.001, closest_t) {
                closest_t = rec.t;
                light = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -150.0);
                color = sphere.get_color_shadow(rec.point, light, 0xFFFFFF)
            }
        }

        *pixel = color;
    });

    buffer
}

fn main() {
    let vector_r = Vector::new(WIDTH as f32 / 2.0 - 50.0, HEIGHT as f32 / 2.0, 10_f32);
    let vector_g = Vector::new(WIDTH as f32 / 2.0 + 50.0, HEIGHT as f32 / 2.0, 20_f32);
    let vector_b = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 86.6, 40_f32);

    let scene = Scene {
        spheres: vec![
            Sphere::new(vector_r, 100.0, 0xFF0000), // Red
            Sphere::new(vector_g, 60.0, 0x00FF00), // Green
            Sphere::new(vector_b, 100.0, 0x0000FF), // Blue
        ],
    };

    let mut window = Window::new("Scene", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap();

    let frame = render_scene(&scene);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}
