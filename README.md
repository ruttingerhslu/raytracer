# Raytracer in Rust

To run (model specified in config.toml):
``cargo run --release -- --model box_vertex_colors``

To run with specific scene, model, camera angle
``cargo run --release -- --model suzanne --angle 45.0 --scene custom``

To animate the scene with specified model:
``cargo run --release -- --model suzanne --angle 45.0 --scene custom --animate``

and then create a gif from the generated folder:
``ffmpeg -framerate 24 -i animation/frame_%03d.ppm.ppm -vf "palettegen" palette.png``
and
``ffmpeg -framerate 24 -i animation/frame_%03d.ppm.ppm -i palette.png -lavfi "paletteuse" animation.gif``

