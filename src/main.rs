use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

use vector::Vector;
use circle::Circle;

mod vector;
mod circle;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Scene {
    circles: Vec<Circle>,
}

fn render_scene(scene: &Scene) -> Vec<u32> {
    let mut buffer = vec![0x000000; WIDTH * HEIGHT];

    buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
        let x = (index % WIDTH) as f32;
        let y = (index / WIDTH) as f32;
        let point = Vector::new(x, y);

        let mut color = 0x000000;

        for circle in &scene.circles {
            if circle.contains(point) {
                color |= circle.get_color(); // Merge colors if inside multiple circles
            }
        }

        *pixel = color;
    });

    buffer
}

fn main() {
    let vector_r = Vector::new(WIDTH as f32 / 2.0 - 50.0, HEIGHT as f32 / 2.0);
    let vector_g = Vector::new(WIDTH as f32 / 2.0 + 50.0, HEIGHT as f32 / 2.0);
    let vector_b = Vector::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 86.6);

    let scene = Scene {
        circles: vec![
            Circle::new(vector_r, 100.0, 0xFF0000), // Red
            Circle::new(vector_g, 100.0, 0x00FF00), // Green
            Circle::new(vector_b, 100.0, 0x0000FF), // Blue
        ],
    };

    let mut window = Window::new("RGB Overlapping Circles", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let frame = render_scene(&scene);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}
