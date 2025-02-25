use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

use vector::Vector;

mod vector;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Scene {
    circle_x: usize,
    circle_y: usize,
    radius: usize,
    circle_color: u32,
}

fn render_scene(scene: &Scene) -> Vec<u32> {
    let mut buffer = vec![0x000000; WIDTH * HEIGHT];

    buffer.par_iter_mut().enumerate().for_each(|(index, pixel)| {
        let x = (index % WIDTH) as f32;
        let y = (index / WIDTH) as f32;

        let dx = x - scene.circle_x as f32;
        let dy = y - scene.circle_y as f32;

        if dx * dx + dy * dy <= (scene.radius as f32).powi(2) {
            *pixel = scene.circle_color;
        }
    });

    buffer
}

fn main() {
    let scene = Scene {
        circle_x: WIDTH / 2,
        circle_y: HEIGHT / 2,
        radius: 50,
        circle_color: 0xFFC0CB,
    };

    let vector_c = Vector::new(scene.circle_x as f32, scene.circle_y as f32);
    let vector_radius = Vector::new(50.0_f32, 0.0_f32);
    let vector_p = vector_c + vector_radius;

    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut frame = render_scene(&scene);

        vector_p.draw(&mut frame, 0x00FF00, WIDTH as f32, HEIGHT as f32);

        window.update_with_buffer(&frame, WIDTH, HEIGHT).unwrap();
    }
}
