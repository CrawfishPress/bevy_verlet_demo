/*
Note that `add_a_circle` starts as a System, so the syntax of Resources
like Command, is wildly different from a function that takes a *reference*.

*/

use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};
use crate::sprites::*;

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
    commands // I'll have to Circle back...
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-475.0, 0.0, 1.0)),
            mesh: meshes.add(shape::Circle::new(20.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()})
        .insert(OneCircle);
    println!("added a circle somewhere []");
}
