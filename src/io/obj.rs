use tobj;
use std::sync::Arc;
use anyhow::Result;
use std::path::PathBuf; 
use std::path::Path; 
use std::collections::HashMap;

use crate::core::vec3::{Vec3, Point3};
use crate::core::color::Color;

use crate::material::material::{Material, TexturedMaterial, Metal};
use crate::material::texture::Texture;                                                                                                                                                                  

use crate::objects::triangle::Triangle;
use crate::objects::world::World;

pub async fn load_obj_from_path(path: &PathBuf, world: &mut World, mat: Arc<dyn Material>, rotation: Vec3) -> Result<(Point3, Point3)> {
    let (models, materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;
    let materials = materials.expect("Failed to load MTL file");

    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut material_map: HashMap<usize, Arc<dyn Material>> = HashMap::new();

    for (i, mat) in materials.iter().enumerate() {
        if let Some(ref map_kd) = mat.diffuse_texture {
            let texture_path = base_dir.join(map_kd);
            if texture_path.exists() {
                let img = image::open(texture_path)?;
                let img = img.to_rgba8();
                let texture = Arc::new(Texture::new(img));
                let textured_material = Arc::new(TexturedMaterial::new(texture));
                material_map.insert(i, textured_material);
            } else {
                eprintln!("Warning: Texture not found: {:?}", texture_path);
            }
        } else {
            let default_mat = Arc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.3));
            material_map.insert(i, default_mat);
        }
    }

    let mut min = Point3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Point3::new(f32::MIN, f32::MIN, f32::MIN);
    let mut center = Vec3::ZERO;

    for model in models {
        let mesh = &model.mesh;

        let positions = mesh.positions.chunks(3)
            .map(|p| {
                let point = Point3::new(p[0], p[1], p[2]);
                min = Point3::new(min.x().min(point.x()), min.y().min(point.y()), min.z().min(point.z()));
                max = Point3::new(max.x().max(point.x()), max.y().max(point.y()), max.z().max(point.z()));
                center = (min + max) * 0.5;
                point
            })
            .collect::<Vec<_>>();

        let texcoords = mesh.texcoords.chunks(2)
            .map(|t| (t[0], t[1]))
            .collect::<Vec<_>>();

        let indices = &mesh.indices;

        let material_id = mesh.material_id;
        let selected_material = material_id
            .and_then(|id| material_map.get(&id).cloned())
            .unwrap_or_else(|| mat.clone());

        for triangle in indices.chunks(3) {
            if triangle.len() == 3 {
                let i0 = triangle[0] as usize;
                let i1 = triangle[1] as usize;
                let i2 = triangle[2] as usize;

                let a = positions[i0].rotate_xyz(rotation) + center;
                let b = positions[i1].rotate_xyz(rotation) + center;
                let c = positions[i2].rotate_xyz(rotation) + center;

                let uv0 = texcoords.get(i0).cloned().unwrap_or((0.0, 0.0));
                let uv1 = texcoords.get(i1).cloned().unwrap_or((0.0, 0.0));
                let uv2 = texcoords.get(i2).cloned().unwrap_or((0.0, 0.0));

                world.add_hittable(Arc::new(Triangle::new(
                    a, b, c,
                    uv0, uv1, uv2,
                    selected_material.clone()
                )));
            }
        }
    }

    Ok((min, max))
}
