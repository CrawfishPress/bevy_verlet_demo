use bevy::{prelude::*};
use bevy::asset::{AssetServer, Handle, Asset};
use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::ecs::prelude::{Commands, Res};
use crate::bitmaps::*;

#[derive(Component, Debug)]
pub struct KeyMover {
    pub is_movable: bool
}

pub struct SpritesMovable {
    pub is_active: bool
}

#[derive(Component)]
pub struct BackgroundMap;

pub fn do_movement_input(keyboard_input: Res<Input<KeyCode>>,
                         mut tunnel_pos: Query<(&mut Transform, &mut KeyMover)>,
                         move_active: Res<SpritesMovable>,)
{
    if !move_active.is_active {return;}

    // For some reason, left-CTRL key not registering - maybe Linux is eating it?
    if keyboard_input.any_pressed([KeyCode::LControl, KeyCode::RControl, KeyCode::LAlt, KeyCode::RAlt]) {
        println!("*** keys pressed: {:?}", keyboard_input);
        return;}

    for (mut tunnel_dir, tunnel_move) in &mut tunnel_pos {
        if tunnel_move.is_movable == false {
            continue;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            tunnel_dir.translation.x -= 7.5
        } else if keyboard_input.pressed(KeyCode::Down) {
            tunnel_dir.translation.y -= 7.5
        } else if keyboard_input.pressed(KeyCode::Up) {
            tunnel_dir.translation.y += 7.5
        } else if keyboard_input.pressed(KeyCode::Right) {
            tunnel_dir.translation.x += 7.5
        }
    }
}
