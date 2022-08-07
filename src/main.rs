/*
Basic example of Verlet equations-of-motion, in Rust/Bevy
*/

use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod bitmaps;
mod sprites;
mod verlet;
mod data;

use bitmaps::*;
use sprites::*;
use verlet::*;
use data::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("{} - v{}",
                           env!("CARGO_PKG_NAME"),
                           env!("CARGO_PKG_VERSION")
            ),
            width: WIDTH,
            height: HEIGHT,
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND))
        .insert_resource(SpritesMovable { is_active: true })
        .insert_resource(CircleTimer(Timer::from_seconds(CIRCLE_DELAY, true)))
        .insert_resource(BallsInGame { total_balls: BALLS_MAX })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_sprites)
        .add_system(add_many_circles)
        .add_system(bevy::window::close_on_esc)
        .add_system(do_movement_input)
        .add_system(solve_for_verlet)
        .run();
    println!("this is a test of the Bevy Engine - alas, this line is never reached, due to a bug");
}

fn setup_sprites(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands  // Camera
        .spawn_bundle(Camera2dBundle::default());

    add_background(&mut commands, &asset_server, "circle.png");

    commands // Center Pixel for Calibration
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-475.0, 0.0, 5.0)),
            mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()});

    commands // And this is my other Center Pixel for Calibration
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-900.0, -425.0, 6.0)),
            mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()});

    commands // Square for Calibration
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-900.0, -425.0, 4.0)),
            mesh: meshes.add(shape::RegularPolygon::new(20.0, 4).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()});

    commands  // Moving Tunnel
        .spawn().insert_bundle(SpriteBundle {
        transform: Transform::from_xyz(80.0, 0.0, 3.0),
        sprite: Sprite {
            color: PADDLE_COLOR,
            custom_size: Option::from(Vec2 { x: BLOCK_SIZE * 2.0, y: BLOCK_SIZE }),
            ..default()
        },
        ..default()})
        .insert(KeyMover {is_movable: true});
}
