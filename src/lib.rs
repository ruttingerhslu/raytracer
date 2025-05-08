pub mod core {
    pub mod color;
    pub mod vec3;
    pub mod ray;
    pub mod camera;
    pub mod common;
}

pub mod objects {
    pub mod sphere;
    pub mod triangle;
    pub mod hittable;
    pub mod world;
    pub mod light;
}

pub mod material {
    pub mod material;
    pub mod texture;
}

pub mod io {
    pub mod obj;
    pub mod asset_loader;
}

pub mod renderer {
    pub mod scene;
    pub mod renderer;
}
