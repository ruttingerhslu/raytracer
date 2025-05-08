mod app;
use anyhow::Result;
use clap::Parser; 

use app::args::Args;
use app::config::load_config;
use app::run::run_or_animate;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config)?;
    run_or_animate(args, config).await
}
