/*
Note that `add_a_circle` starts as a System, so the syntax of Resources
like Command, is wildly different from a function that takes a *reference*.

In the Python/Kivy version of this, I created a Vector2 class, but
Bevy already has Vec2, which is stored in the sprite's Transform.
*/

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};

use crate::sprites::*;
use crate::verlet::*;
use crate::data::*;

#[derive(Component, Debug)]
pub struct OneCircle;

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
                        action_check: Res<PitActive>,
                        mut commands: Commands,
                        mut meshes: ResMut<Assets<Mesh>>,
                        mut materials: ResMut<Assets<ColorMaterial>>,
                        mut timer: ResMut<CircleTimer>,
                        mut balls_left: ResMut<BallsInGame>,
                        mut random_data: ResMut<GuiData>,
){
    if action_check.is_paused { return; }
    if timer.0.paused() { return; }
    timer.0.tick(time.delta());
    if ! timer.0.finished() { return; }

    // TODO: make the x/y offset, random
    let ball_radius = random_data.radius_slider_value;
    add_a_circle(&mut commands, &mut meshes, &mut materials, ball_radius, 200.0, 200.0);

    balls_left.balls_added += 1;
    random_data.total_balls = balls_left.balls_added;

    if balls_left.balls_added >= random_data.ball_slider_value {
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
        .insert(OneCircle)
        .insert(KeyMover {is_movable: true})
        .insert(verlet_data);
}
