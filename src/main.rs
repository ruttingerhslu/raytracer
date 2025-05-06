#![allow(unused)]

mod asset_loader;

use anyhow::{Result};
use std::collections::HashMap;
use std::fs;
use clap::Parser;
use serde::Deserialize;

use raytracer::scenes::custom_scene;
use raytracer::scenes::required_scene;

use raytracer::scene::Scene;
use raytracer::world::World; use asset_loader::download_obj_with_assets;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    model: String,

    #[arg(short, long, default_value = "config.toml")]
    config: String,
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
    let camera = custom_scene::setup_custom_scene(&obj_path, &mut world).await?;
    // let camera = required_scene::setup_scene(&obj_path, &mut world).await?;

    let scene = Scene { camera };
    Scene::render_scene(&scene, world, WIDTH, HEIGHT);

    Ok(())
}
