use rayon::prelude::*;
use std::fs::{File, rename};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::vec3::{self};
use crate::ray::Ray;
use crate::camera::Camera;
use crate::common;

const SAMPLES_PER_PIXEL: i32 = 10;
const MAX_DEPTH: i32 = 5;

pub struct Scene {
    pub camera: Camera,
}

impl Scene {
    fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, 0.001, common::INFINITY, &mut rec) {
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();
            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render_scene(&self, world: HittableList, width: usize, height: usize) {
        let tmp_path = Path::new("output.tmp.ppm");
        let final_path = Path::new("output.ppm");

        let file = File::create(&tmp_path).expect("Failed to create temp file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3").expect("Failed to write PPM header");
        writeln!(writer, "{} {}", width, height).expect("Failed to write dimensions");
        writeln!(writer, "255").expect("Failed to write max color value");

        let camera = Arc::new(self.camera.clone());
        let world = Arc::new(world);
        let progress = Arc::new(AtomicUsize::new(0));

        let pixel_data: Vec<String> = (0..height)
            .into_par_iter()
            .rev()
            .map(|j| {
                let progress = Arc::clone(&progress);
                let mut scanline = Vec::with_capacity(width);
                for i in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let mod_x = i as f32 + common::random_double();
                        let mod_y = j as f32 + common::random_double();
                        let u = mod_x / (width - 1) as f32;
                        let v = mod_y / (height - 1) as f32;
                        let r = camera.get_ray(u, v);
                        pixel_color += Self::ray_color(&r, &*world, MAX_DEPTH);
                    }
                    scanline.push(color::format_color(pixel_color, SAMPLES_PER_PIXEL));
                }
                let completed = progress.fetch_add(1, Ordering::Relaxed);
                eprint!("\rScanlines completed: {}/{}", completed + 1, height);
                scanline
            })
            .flatten()
            .collect();

        for line in &pixel_data {
            writeln!(writer, "{}", line).expect("Failed to write pixel");
        }

        writer.flush().expect("Failed to flush buffer");
        rename(tmp_path, final_path).expect("Failed to rename temp file");
        eprint!("\nDone. Image saved to output.ppm\n");
    }
}

