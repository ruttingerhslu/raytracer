use rayon::prelude::*;
use std::fs::{File, rename};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs::create_dir_all; 

use crate::core::color::{self, Color};
use crate::core::vec3::{self};
use crate::core::ray::Ray;
use crate::core::camera::Camera;
use crate::core::common;

use crate::objects::world::World;
use crate::objects::hittable::HitRecord;

const SAMPLES_PER_PIXEL: i32 = 20;
const MAX_DEPTH: i32 = 10;

pub struct Renderer {
    pub camera: Camera,
    pub world: World,
}

impl Renderer {
    pub fn new(camera: Camera, world: World) -> Self {
        Self {
            camera: camera,
            world: world,
        }
    }

    fn ray_color(r: &Ray, world: &World, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, 0.001, common::INFINITY, &mut rec) {
            let mut direct_light = Color::new(0.0, 0.0, 0.0);

            for light in &world.lights {
                let light_dir = (light.position() - rec.p).normalize();
                let light_distance = (light.position() - rec.p).length();
                let shadow_ray = Ray::new(rec.p, light_dir);
                let mut shadow_rec = HitRecord::new();
                let in_shadow = world.hit(&shadow_ray, 0.001, light_distance, &mut shadow_rec);

                if !in_shadow {
                    // Diffuse shading (Lambert)
                    let diffuse_intensity = vec3::dot(light_dir, rec.normal).max(0.0);
                    let diffuse = rec.mat.as_ref().unwrap().albedo() * diffuse_intensity;

                    // Specular
                    let view_dir = -r.direction().normalize();
                    let reflect_dir = vec3::reflect(-light_dir, rec.normal).normalize();
                    let spec_strength = vec3::dot(reflect_dir, view_dir).max(0.0).powf(32.0);
                    let specular_color = Color::new(1.0, 1.0, 1.0);
                    let specular = specular_color * spec_strength;

                    // Light attenuation
                    let attenuation = 1.0 / (light_distance * light_distance + 1.0);
                    let contribution = (diffuse + specular) * light.intensity() * attenuation;
                    direct_light += contribution;
                }
            }

            // Ambient Light (konstantes Grundlicht)
            let ambient = rec.mat.as_ref().unwrap().albedo() * 0.1;

            // Recursive scattering (reflection, refraction)
            let mut indirect_light = Color::new(0.0, 0.0, 0.0);
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();
            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                indirect_light += attenuation * Self::ray_color(&scattered, world, depth - 1);
            }

            return ambient + direct_light + indirect_light * 0.8;
        }

        // Background gradient
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render_scene(&self, width: usize, height: usize) {
        let tmp_path = Path::new("output.tmp.ppm");
        let final_path = Path::new("output.ppm");

        let file = File::create(&tmp_path).expect("Failed to create temp file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3").expect("Failed to write PPM header");
        writeln!(writer, "{} {}", width, height).expect("Failed to write dimensions");
        writeln!(writer, "255").expect("Failed to write max color value");

        let camera = Arc::new(self.camera.clone());
        let world = Arc::new(self.world.clone());
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

    pub fn render_scene_to_file(&self, width: usize, height: usize, filename: &str) {
        let output_dir = Path::new("animation");
        create_dir_all(output_dir).expect("Failed to create output directory");

        let tmp_path = output_dir.join(format!("{filename}.tmp"));
        let final_path = output_dir.join(format!("{filename}.ppm"));

        let file = File::create(&tmp_path).expect("Failed to create temp file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3").expect("Failed to write PPM header");
        writeln!(writer, "{} {}", width, height).expect("Failed to write dimensions");
        writeln!(writer, "255").expect("Failed to write max color value");

        let camera = Arc::new(self.camera.clone());
        let world = Arc::new(self.world.clone());

        let pixel_data: Vec<String> = (0..height)
            .into_par_iter()
            .rev()
            .map(|j| {
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
                scanline
            })
            .flatten()
            .collect();

        for line in &pixel_data {
            writeln!(writer, "{}", line).expect("Failed to write pixel");
        }

        writer.flush().expect("Failed to flush buffer");
        std::fs::rename(tmp_path, &&final_path).expect("Failed to rename temp file");
    }
}

