use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::fs;
use anyhow::Result;

use crate::app::args::Args;
use raytracer::objects::world::World;
use raytracer::renderer::scene::Scene;
use raytracer::renderer::renderer::Renderer;

const NUM_FRAMES: usize = 60;

pub async fn render_animation(args: Args, scene: Box<dyn Scene>, obj_path: &PathBuf, width: usize, height: usize) -> Result<()> {
    let animation_filename = format!("animation/{}_{}.gif", args.scene, args.model);

    for frame in 0..NUM_FRAMES {
        let angle = frame as f32 / NUM_FRAMES as f32 * std::f32::consts::TAU;
        let mut world = World::new();
        let camera = scene.setup(obj_path, &mut world, angle).await?;
        let renderer = Renderer::new(camera, world);
        let filename = format!("frame_{:03}", frame);
        renderer.render_scene_to_file(width, height, &filename);
    }

    Command::new("ffmpeg")
        .args(["-y", "-framerate", "24", "-i", "animation/frame_%03d.ppm", "-vf", "palettegen", "palette.png"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate", "24",
            "-i", "animation/frame_%03d.ppm",
            "-i", "palette.png",
            "-lavfi", "paletteuse",
            &animation_filename,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    for entry in fs::read_dir("animation")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("ppm") {
            fs::remove_file(path)?;
        }
    }

    fs::remove_file("palette.png").ok();

    println!("Animation saved to {}", animation_filename);
    Ok(())
}
