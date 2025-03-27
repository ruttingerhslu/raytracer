pub fn to_rgb(color: u32) -> (f32, f32, f32) {
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;
    (r, g, b)
}

pub fn from_rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}

pub fn apply_intensity(color: u32, intensity: f32) -> u32 {
    let (r, g, b) = to_rgb(color);
    let r = (r * intensity * 255.0).min(255.0) as u32;
    let g = (g * intensity * 255.0).min(255.0) as u32;
    let b = (b * intensity * 255.0).min(255.0) as u32;
    from_rgb(r, g, b)
}

pub fn apply_intensity_with_color(object_color: u32, light_color: u32, intensity: f32) -> u32 {
    let (r_obj, g_obj, b_obj) = to_rgb(object_color);
    let (r_light, g_light, b_light) = to_rgb(light_color);

    let r = (r_obj * r_light * intensity).min(255.0) as u32;
    let g = (g_obj * g_light * intensity).min(255.0) as u32;
    let b = (b_obj * b_light * intensity).min(255.0) as u32;

    from_rgb(r, g, b)
}

pub fn apply_ambient(color: u32, light_color: u32, intensity: f32) -> u32 {
    let (r_obj, g_obj, b_obj) = to_rgb(color);
    let (r_light, g_light, b_light) = to_rgb(light_color);

    let r = (r_obj * r_light * intensity * 255.0) as u32;
    let g = (g_obj * g_light * intensity * 255.0) as u32;
    let b = (b_obj * b_light * intensity * 255.0) as u32;

    from_rgb(r, g, b)
}

pub fn apply_diffuse(color: u32, light_color: u32, intensity: f32, cos_delta: f32) -> u32 {
    let (r_obj, g_obj, b_obj) = to_rgb(color);
    let (r_light, g_light, b_light) = to_rgb(light_color);

    let r = (r_obj * r_light * intensity * cos_delta * 255.0) as u32;
    let g = (g_obj * g_light * intensity * cos_delta * 255.0) as u32;
    let b = (b_obj * b_light * intensity * cos_delta * 255.0) as u32;

    from_rgb(r, g, b)
}

pub fn combine_colors(ambient: u32, diffuse: u32) -> u32 {
    let (r_a, g_a, b_a) = to_rgb(ambient);
    let (r_d, g_d, b_d) = to_rgb(diffuse);

    let r = (r_a + r_d).min(1.0) * 255.0;
    let g = (g_a + g_d).min(1.0) * 255.0;
    let b = (b_a + b_d).min(1.0) * 255.0;

    from_rgb(r as u32, g as u32, b as u32)
}
