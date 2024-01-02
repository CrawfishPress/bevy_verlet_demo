/*
Basic example of Verlet equations-of-motion, in Rust/Bevy
*/

use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle};
use bevy::ecs::prelude::{Commands, Res};

#[allow(unused_imports)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::WindowResolution;

#[allow(unused_imports)]
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod balls;
mod keymovers;
mod verlet;
mod data;
mod my_egui;

use balls::*;
use keymovers::*;
use verlet::*;
use data::*;
use my_egui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("{} - v{}",
                               env!("CARGO_PKG_NAME"),
                               env!("CARGO_PKG_VERSION")
                ),
                mode: WindowMode::Windowed,
                resizable: true,
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND))

        .insert_resource(GuiData::default())
        .insert_resource(SpritesMovable { is_active: true })
        .insert_resource(CircleTimer(Timer::from_seconds(CIRCLE_DELAY_MILLIS as f32 / 1000.0, TimerMode::Repeating)))
        .insert_resource(PitActive::default())

        .add_plugins(EguiPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_systems(Startup, setup_sprites)
        .add_systems(Startup, configure_visuals_system)

        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, do_ui_setup)
        .add_systems(Update, add_many_circles)
        .add_systems(Update, remove_circles)
        .add_systems(Update, do_movement_input)
        .add_systems(Update, solve_for_verlet)
        .run();
    println!("this is a test of the Bevy Engine - alas, this line is never reached, due to a bug");
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn setup_sprites(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands  // Camera
        .spawn(Camera2dBundle::default());

    add_background(&mut commands, &asset_server, "circle.png");
    // add_background(&mut commands, &asset_server, "/static/web/assets/circle.png");  // For WASM

    commands // Center Pixel for Calibration
        .spawn(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-475.0, 0.0, 5.0)),
            mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()});

    commands // And this is my other Center Pixel for Calibration
        .spawn(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-900.0, -425.0, 6.0)),
            mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()});

    commands // Square for Calibration
        .spawn(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-900.0, -425.0, 4.0)),
            mesh: meshes.add(shape::RegularPolygon::new(20.0, 4).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()});

    commands  // Moving Tunnel
        .spawn(SpriteBundle {
        transform: Transform::from_xyz(80.0, 0.0, 3.0),
        sprite: Sprite {
            color: PADDLE_COLOR,
            custom_size: Option::from(Vec2 { x: BLOCK_SIZE * 2.0, y: BLOCK_SIZE }),
            ..default()
        },
        ..default()})
        .insert(KeyMover {is_movable: true});
}
