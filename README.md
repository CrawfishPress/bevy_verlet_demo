### kivy_verlet_sample

Inspired by the excellent video:

- https://www.youtube.com/watch?v=lS_qeBy3aQI
- (How to program a basic Physic Engine by Pezzza's Work)

This is a quick demo of the basic Verlet equations for object-motion,
implemented in Rust + Bevy. (Uses Rust 1.62.1/Bevy 0.8.0)

### Currently
 - one circular constraint enabled
 - multi-ball collisions working
 - ball-release on timer (fairly slowly, or incoming balls smash previous ball)
 - collision-damping funky, some popcorn-effects at times

### TODO
 - figure out how to spawn balls on timer, without instant collisions
 - experiment with dampening, to get less "dynamic" collisions

### To Run
You don't have to use `release`, but a `debug` compile, takes a few seconds
longer to load the background bitmap.

    cargo run --release

### Output
![](Screenshot.png)