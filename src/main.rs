#![allow(unused)]

mod asset_loader;

use anyhow::{Result};
use std::collections::HashMap;
use std::fs;
use clap::Parser;
use serde::Deserialize;
use std::process::{Command, Stdio};

use raytracer::scene::{Scene, CustomScene, RequiredScene, MuseumScene}; 

use raytracer::renderer::Renderer;
use raytracer::world::World; 
use asset_loader::download_obj_with_assets;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    model: String,

    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(long, default_value = "custom")]
    scene: String,

    #[arg(long, default_value_t = 20.0)]
    angle: f32,

    #[arg(long)]
    animate: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    models: HashMap<String, String>,
}

fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config)?;

    let Some(url) = config.models.get(&args.model) else {
        eprintln!("Model '{}' not found in config", args.model);
        std::process::exit(1);
    };

    let (_tmp_dir, obj_path) = download_obj_with_assets(url).await?;
    let mut world = World::new();

    // set desired scene
    let scene: Box<dyn Scene> = match args.scene.as_str() {
        "custom" => Box::new(CustomScene),
        "required" => Box::new(RequiredScene),
        "museum" => Box::new(MuseumScene),
        _ => Box::new(CustomScene),
    };
    let angle = args.angle;

    if !args.animate {
        let camera = scene.setup(&obj_path, &mut world, angle).await?;
        let renderer = Renderer::new(camera, world);
        renderer.render_scene(WIDTH, HEIGHT);
    } else {
        let animation_filename = format!("animation/{}_{}.gif", args.scene, args.model);

        // animation
        const NUM_FRAMES: usize = 60;

        for frame in 0..NUM_FRAMES {
            let angle = frame as f32 / NUM_FRAMES as f32 * std::f32::consts::TAU; // full rotation
            let mut world = World::new();
            let camera = scene.setup(&obj_path, &mut world, angle).await?;
            let renderer = Renderer::new(camera, world);
            let filename = format!("frame_{:03}", frame);
            renderer.render_scene_to_file(WIDTH, HEIGHT, &filename);
        }

        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate", "24",
                "-i", "animation/frame_%03d.ppm",
                "-vf", "palettegen",
                "palette.png",
            ])
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

        println!("✅ Animation saved to {}", animation_filename);
    }

    Ok(())
}
