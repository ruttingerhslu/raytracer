use anyhow::Result;

use crate::app::args::Args;
use crate::app::config::Config;
use crate::app::animation::render_animation;

use raytracer::io::asset_loader::download_obj_with_assets;
use raytracer::objects::world::World;
use raytracer::renderer::scene::{CustomScene, MuseumScene, RequiredScene, Scene};
use raytracer::renderer::renderer::Renderer;

pub async fn run_or_animate(args: Args, config: Config) -> Result<()> {
    let Some(url) = config.models.get(&args.model) else {
        eprintln!("Model '{}' not found in config", args.model);
        std::process::exit(1);
    };

    let (_tmp_dir, obj_path) = download_obj_with_assets(url).await?;
    let mut world = World::new();

    let scene: Box<dyn Scene> = match args.scene.as_str() {
        "custom" => Box::new(CustomScene),
        "required" => Box::new(RequiredScene),
        "museum" => Box::new(MuseumScene),
        _ => Box::new(CustomScene),
    };

    let width = args.resolution;
    let height = width;

    if !args.animate {
        let camera = scene.setup(&obj_path, &mut world, args.angle).await?;
        let renderer = Renderer::new(camera, world);
        renderer.render_scene(width, height);
    } else {
        render_animation(args, scene, &obj_path, width, height).await?;
    }

    Ok(())
}
