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

#[derive(Component)]
pub struct BackgroundMap;

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
            ..default()})
        .insert(BackgroundMap);
    // println!("added background [{}]", some_bitmap);
}

pub fn add_many_circles(mut commands: Commands,
                        mut meshes: ResMut<Assets<Mesh>>,
                        mut materials: ResMut<Assets<ColorMaterial>>,
                        mut timer: ResMut<CircleTimer>,
                        time: Res<Time>,
                        mut balls_left: ResMut<BallsInGame>,
){
    if timer.0.paused() { return; }
    timer.0.tick(time.delta());
    if ! timer.0.finished() { return; }

    add_a_circle(&mut commands, &mut meshes, &mut materials, 200.0, 200.0);

    balls_left.total_balls -= 1;
    if balls_left.total_balls <= 0 {
        timer.0.pause();
    }

    //add_a_circle(&mut commands, &mut meshes, &mut materials, 200.0, 0.0);
    //add_a_circle(&mut commands, &mut meshes, &mut materials, 300.0, 200.0);
}

pub fn add_a_circle(commands: &mut Commands,
                    meshes: &mut ResMut<Assets<Mesh>>,
                    materials: &mut ResMut<Assets<ColorMaterial>>,
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
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()})
        .insert(OneCircle)
        .insert(KeyMover {is_movable: true})
        .insert(verlet_data);
    // println!("added a circle somewhere. I think it's at {}", circle_vec3);
}
