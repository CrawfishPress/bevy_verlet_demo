### kivy_verlet_sample

Inspired by the excellent video:

- https://www.youtube.com/watch?v=lS_qeBy3aQI
- (How to program a basic Physic Engine by Pezzza's Work)

This is a quick demo of the basic Verlet equations for object-motion,
implemented in Rust + Bevy. (Uses Rust 1.62.1/Bevy 0.8.0)

Note - I did another version of this in Python/Kivy, but not surprisingly,
it was only able to handle 100-150 balls without dropping framerate. The
Rust/Bevy version handled 500+ balls at 60 FPS, that I've tested.

### Currently
 - one circular constraint enabled
 - multi-ball collisions working
 - ball-release on timer (fairly slowly, or incoming balls smash previous ball)
 - collision-damping funky, some popcorn-effects at times
 - at 500 balls, holding at 60 FPS - and I haven't added the pre-collision filtering

### TODO
 - figure out how to spawn balls on timer, without instant collisions
 - experiment with dampening, to get less "dynamic" collisions
 - change balls to have different radii
 - add static balls for pinball-effect
 - add links for some chaining
 - add control-panel for modifying parameters without re-compiling

### To Run
You don't have to use `release`, but a `debug` compile, takes a few seconds
longer to load the background bitmap.

    cargo run --release

### Output
![](Screenshot.png)