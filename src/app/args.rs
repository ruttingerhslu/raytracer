use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = "suzanne")]
    pub model: String,

    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    #[arg(long, default_value = "custom")]
    pub scene: String,

    #[arg(long, default_value_t = 90.0)]
    pub angle: f32,

    #[arg(long)]
    pub animate: bool,
}
