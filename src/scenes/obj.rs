use tobj;
use std::sync::Arc;
use anyhow::Result;
use std::path::PathBuf; 

use crate::triangle::Triangle;
use crate::vec3::{Point3};
use crate::world::World;
use crate::material::{Material};

pub async fn load_obj_from_path(path: &PathBuf, world: &mut World, mat: Arc<dyn Material>) -> Result<(Point3, Point3)> {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let mut min = Point3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Point3::new(f32::MIN, f32::MIN, f32::MIN);


    for (model_idx, model) in models.iter().enumerate() {
        let mesh = &model.mesh;
        println!("Model {}: name = {}", model_idx, model.name);
        println!(" - Positions: {} vertices", mesh.positions.len() / 3);
        println!(" - Indices: {} indices ({} triangles)", mesh.indices.len(), mesh.indices.len() / 3);
        println!(" - Texcoords: {}", mesh.texcoords.len() / 2);

        let positions = mesh.positions.chunks(3)
            .map(|p| {
                let point = Point3::new(p[0], p[1], p[2]);
                min = Point3::new(min.x().min(point.x()), min.y().min(point.y()), min.z().min(point.z()));
                max = Point3::new(max.x().max(point.x()), max.y().max(point.y()), max.z().max(point.z()));
                point
            })
            .collect::<Vec<_>>();

        let texcoords = mesh.texcoords.chunks(2)
            .map(|t| (t[0], t[1]))
            .collect::<Vec<_>>();

        let indices = &mesh.indices;

        for triangle in indices.chunks(3) {
            if triangle.len() == 3 {
                let i0 = triangle[0] as usize;
                let i1 = triangle[1] as usize;
                let i2 = triangle[2] as usize;

                let a = positions[i0];
                let b = positions[i1];
                let c = positions[i2];

                let uv0 = texcoords.get(i0).cloned().unwrap_or((0.0, 0.0));
                let uv1 = texcoords.get(i1).cloned().unwrap_or((0.0, 0.0));
                let uv2 = texcoords.get(i2).cloned().unwrap_or((0.0, 0.0));

                world.add_hittable(Box::new(Triangle::new(
                    a, b, c,
                    uv0, uv1, uv2,
                    mat.clone()
                )));
            }
        }
    }

    Ok((min, max))
}
