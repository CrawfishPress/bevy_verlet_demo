/*
Technically, the Sprites are in bitmaps.rs, this file just contains
the motions-functions.

Note that the Bevy Query, returns a smart-pointer - figuring out how
to unwrap that
    Mut<'_, bevy::prelude::Transform, >
was tricky.
*/

use bevy::{prelude::*};
use bevy::ecs::prelude::{Res};

use crate::verlet::*;

fn print_type_of<T>(_: &T) {print!("{}", std::any::type_name::<T>())} // Unstable.

#[derive(Component)]
pub struct KeyMover {
    pub is_movable: bool
}

pub struct SpritesMovable {
    pub is_active: bool
}

pub fn do_movement_input(keyboard_input: Res<Input<KeyCode>>,
                         move_active: Res<SpritesMovable>,
                         mut tunnel_pos: Query<(&mut Transform, &mut KeyMover)>,
){
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

pub fn solve_for_verlet(mut balls_qry: Query<(Entity, &mut VerletData, &mut Transform)>,
){
    for (entity_id, mut verlet_data, mut pos_vec) in &mut balls_qry {
        // println!("balls-ahoy: {:?}: {:?}", entity_id, verlet_data);
        apply_gravity(&mut verlet_data, pos_vec);
    }
}

pub fn apply_gravity(verlet_data: &Mut<'_, VerletData>, mut pos_vec: Mut<'_, bevy::prelude::Transform, >)
{
    // println!("balls-data: {:?},\n\t {:?}",  verlet_data, pos_vec);
    // println!();
    pos_vec.translation.y -= 1.0;
}
