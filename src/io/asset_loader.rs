use anyhow::Result;
use tempfile::TempDir;
use std::fs;
use std::path::{PathBuf};
use regex::Regex;

pub async fn download_obj_with_assets(obj_url: &str) -> Result<(TempDir, PathBuf)> {
    let base_url = {
        let mut url = reqwest::Url::parse(obj_url)?;
        url.path_segments_mut().unwrap().pop();
        url.to_string()
    };

    let temp_dir = tempfile::tempdir()?;

    let obj_bytes = reqwest::get(obj_url).await?.bytes().await?;
    let obj_path = temp_dir.path().join("model.obj");
    fs::write(&obj_path, &obj_bytes)?;

    let obj_text = String::from_utf8_lossy(&obj_bytes);
    let mtl_file = obj_text
        .lines()
        .find_map(|line| line.strip_prefix("mtllib "))
        .map(str::trim)
        .map(|s| s.to_string());

    if let Some(mtl_file_name) = mtl_file {
        let mtl_url = format!("{base_url}/{}", mtl_file_name);
        let mtl_bytes = reqwest::get(&mtl_url).await?.bytes().await?;
        let mtl_path = temp_dir.path().join(&mtl_file_name);
        fs::write(&mtl_path, &mtl_bytes)?;

        let mtl_text = String::from_utf8_lossy(&mtl_bytes);
        let tex_re = Regex::new(r"^map_Kd\s+(.+)$").unwrap();

        for line in mtl_text.lines() {
            if let Some(caps) = tex_re.captures(line) {
                let tex_file = caps[1].trim();
                let tex_url = format!("{base_url}/{}", tex_file);
                let tex_bytes = reqwest::get(&tex_url).await?.bytes().await?;
                
                let tex_path = temp_dir.path().join(tex_file);
                
                if let Some(parent) = tex_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&tex_path, &tex_bytes)?;
            }
        }
    }
    Ok((temp_dir, obj_path))
}
