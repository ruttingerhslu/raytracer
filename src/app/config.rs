use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub models: HashMap<String, String>,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
