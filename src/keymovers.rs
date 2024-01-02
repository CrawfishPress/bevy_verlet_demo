/*

*/

use bevy::{prelude::*};
use bevy::ecs::prelude::{Res};

// fn print_type_of<T>(_: &T) {print!("{}", std::any::type_name::<T>())} // Unstable.

#[derive(Component)]
#[derive(Resource)]
pub struct KeyMover {
    pub is_movable: bool
}

#[derive(Resource)]
pub struct SpritesMovable {
    pub is_active: bool
}

pub fn do_movement_input(keyboard_input: Res<Input<KeyCode>>,
                         move_active: Res<SpritesMovable>,
                         mut tunnel_pos: Query<(&mut Transform, &mut KeyMover)>,
){
    if !move_active.is_active {return;}

    // For some reason, left-CTRL key not registering - maybe Linux is eating it?
    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::AltLeft, KeyCode::AltRight]) {
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
