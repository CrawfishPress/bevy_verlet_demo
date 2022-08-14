/*
Note that `add_a_circle` starts as a System, so the syntax of Resources
like Command, is wildly different from a function that takes a *reference*.

In the Python/Kivy version of this, I created a Vector2 class, but
Bevy already has Vec2, which is stored in the sprite's Transform.

Note: the Timer system is interesting, and a bit tricky. I made it
infinitely-repeating, at half-second intervals (configurable),
so every time it reaches a "finished" state, it runs the add-ball code,
and then resets to unfinished/zero ticks.

I keep thinking of removing KeyMover status from the Circles, but it
can be entertaining, to totally spin them up. Has nothing to do with
the Verlet Engine, but fun.

Note: when I added release-delay to a Control-panel slider, I realized
that it can be changed, as the balls are dropping. More importantly, I
then realized that the other values, like ball-count and *radius*, can
also be changed while the balls are being released. I thought about
making these variables static for the duration, but then I decided,
"what the heck..." Not like it's hurting anything. (although the
collision-logic does need to be changed for different radii.
*/

use std::time::Duration;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};

use crate::keymovers::*;
use crate::verlet::*;
use crate::data::*;

pub struct CircleTimer(pub Timer);

pub fn add_background(commands: &mut Commands,
                      asset_server: &Res<AssetServer>,
                      some_bitmap: &str,
){
    commands  // Background Map
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load(some_bitmap),
            ..default()});
}

pub fn add_many_circles(time: Res<Time>,
                        mut commands: Commands,
                        mut meshes: ResMut<Assets<Mesh>>,
                        mut materials: ResMut<Assets<ColorMaterial>>,
                        mut timer: ResMut<CircleTimer>,

                        mut random_data: ResMut<GuiData>,
                        mut action_status: ResMut<PitActive>,
){
    if action_status.is_paused { return; }
    if action_status.game_status != GameState::Running { return; }

    if timer.0.paused() { return; }

    // TODO: do this somewhere else - see Comments
    timer.0.set_duration(Duration::from_millis(random_data.circle_delay));

    timer.0.tick(time.delta()); // Gotta feed a few deltas to the Timer
    if ! timer.0.finished() { return; }

    // TODO: make the x/y offset, random
    let ball_radius = random_data.radius_slider_value;
    add_a_circle(&mut commands, &mut meshes, &mut materials, ball_radius, 200.0, 200.0);

    // Update GUI count, pause timer on all balls added
    action_status.balls_added += 1;
    random_data.total_balls = action_status.balls_added;

    if action_status.balls_added >= random_data.ball_slider_value {
        timer.0.pause();
    }
}

pub fn add_a_circle(commands: &mut Commands,
                    meshes: &mut ResMut<Assets<Mesh>>,
                    materials: &mut ResMut<Assets<ColorMaterial>>,
                    ball_radius: f32,
                    x_offset: f32,
                    y_offset: f32,
){
    let circle_vec3 = Vec3::new(MY_PIT.pit_center.x + x_offset, MY_PIT.pit_center.y + y_offset, 1.0);
    let circle_vec2 = Vec2::new(MY_PIT.pit_center.x + x_offset, MY_PIT.pit_center.y + y_offset);

    let verlet_data = VerletData {pos_current: circle_vec2,
                                  pos_old: circle_vec2,
                                  base_gravity: Vec2 { x: 0.0, y: -1.0 },
                                  acceleration: Vec2 { x: 0.0, y: 0.0 },
                                  delta_t: 0.5,
    };

    commands // I'll have to Circle back...
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(circle_vec3),
            mesh: meshes.add(shape::Circle::new(ball_radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()})
        .insert(KeyMover {is_movable: true})
        .insert(OneCircle)
        .insert(verlet_data);
}

pub fn remove_circles(mut commands: Commands,
                      action_status: Res<PitActive>,
                      mut timer: ResMut<CircleTimer>,
                      mut random_data: ResMut<GuiData>,
                      mut balls_query: Query<Entity, With<OneCircle>>,
) {
    if action_status.game_status != GameState::Reset { return; }

    // Remove all the balls from pit
    for one_circle_id in balls_query.iter_mut() {
        commands.entity(one_circle_id).despawn();
    }

    timer.0.unpause();

    random_data.total_balls = 0;

    // Reset all the variables in PitActive - by replacing it?
    commands.remove_resource::<PitActive>();
    commands.insert_resource(PitActive::default());
}
