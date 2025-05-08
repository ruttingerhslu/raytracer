# 🦀 Raytracer in Rust

This raytracer was developed as part of the Raytracing course at HSLU and is inspired by the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## 📋 Requirements

Before running the raytracer, make sure you have the following installed:

- **Rust**: The raytracer is developed using the Rust programming language. Install it from the [official Rust website](https://www.rust-lang.org/).
  
- **FFmpeg**: Required for converting animation frames to GIFs. Install FFmpeg from the [official FFmpeg website](https://ffmpeg.org/download.html).

Once these are installed, you can proceed with building and running the raytracer.

## 🚀 Running the Raytracer

⚠️ Always use the `--release` flag for significantly faster rendering performance.

### Run with Default Model (from `config.toml`)

```bash
cargo run --release -- --model box_vertex_colors
```

### Run with Custom Scene, Model, and Camera Angle
```bash
cargo run --release -- --model suzanne --angle 45.0 --scene custom
```

### Render an animation
```bash
cargo run --release -- --model suzanne --angle 45.0 --scene custom --animate
```

## 🖼️ Convert Animation to GIF (with ffmpeg)
Essentially, this is done if the `--animate` flag is set, but you can also do it manually.

First, generate a color palette:
```bash
ffmpeg -framerate 24 -i animation/frame_%03d.ppm -vf "palettegen" palette.png
```
Then create the final GIF:
```bash
ffmpeg -framerate 24 -i animation/frame_%03d.ppm -i palette.png -lavfi "paletteuse" animation.gif
```

## Default Settings
If no arguments are passed, the app uses defaults defined in src/app/args.rs:
```bash
cargo run --release
```

