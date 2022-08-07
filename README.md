### kivy_verlet_sample

Inspired by the excellent video:

- https://www.youtube.com/watch?v=lS_qeBy3aQI
- (How to program a basic Physic Engine by Pezzza's Work)

This is a quick demo of the basic Verlet equations for object-motion,
implemented in Rust + Bevy. (Uses Rust 1.62.1/Bevy 0.8.0)

### To Run
You don't have to use `release`, but a `debug` compile, takes a few seconds
longer to load the background bitmap.

    cargo run --release

### Output
![](Screenshot.png)