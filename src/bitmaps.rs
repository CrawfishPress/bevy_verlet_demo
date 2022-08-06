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

#[derive(Component)]
pub struct BackgroundMap;

#[derive(Component, Debug)]
pub struct OneCircle;

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
    println!("added background [{}]", some_bitmap);
}

pub fn add_a_circle(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let circle_vec3 = Vec3::new(-475.0, 0.0, 1.0);
    let verlet_data = VerletData {pos_current: Vec2 { x: -475.0, y: 0.0 },
                                  pos_old: Vec2 { x: -475.0, y: 0.0 },
                                  base_gravity: Vec2 { x: 0.0, y: -0.1 },
    };

    commands // I'll have to Circle back...
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(circle_vec3),
            mesh: meshes.add(shape::Circle::new(20.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()})
        .insert(OneCircle)
        .insert(KeyMover {is_movable: true})
        .insert(verlet_data);
    println!("added a circle somewhere. I think it's at {}", circle_vec3);
}
