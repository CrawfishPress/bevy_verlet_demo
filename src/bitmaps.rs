use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};
use crate::sprites::*;

pub fn add_background(commands: &mut Commands, asset_server: &Res<AssetServer>, some_bitmap: &str)
{
    commands  // Background Map
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load(some_bitmap),
            ..default()})
        .insert(BackgroundMap);
    println!("added background [{}]", some_bitmap);
}
