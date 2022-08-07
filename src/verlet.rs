/*
Note that Bevy's Y-axis goes down the screen - so, like Python.Kivy,
acceleration is reversed.
*/

use bevy::{prelude::*};
use bevy::math::Vec2;

use crate::data::{BALL_RADIUS, MY_PIT};

// fn print_type_of<T>(_: &T) {print!("{}", std::any::type_name::<T>())} // Unstable.

#[derive(Component, Debug)]
pub struct VerletData {
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub base_gravity: Vec2,
    pub acceleration: Vec2,
    pub delta_t: f32,
}

pub fn solve_for_verlet(mut balls_qry: Query<(Entity, &mut VerletData, &mut Transform)>,)
{
    for (_entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        // apply_gravity(verlet_data, entity_pos);  // No hope that this would ever work. See lessons.md
        apply_gravity(&mut verlet_data, &mut *entity_pos);
    }

    for (_entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        apply_constraints(&mut verlet_data, &mut *entity_pos);
    }

    //for (entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
    //    for (entity_id2, mut verlet_data2, mut entity_pos2) in balls_qry.iter_mut() {
    //        println!("E1 = {:?}, E2 = {:?}", entity_id, entity_id2)
    //    }
    //}

    for (_entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        update_position(&mut verlet_data, &mut *entity_pos);
    }

}

// pub fn apply_gravity(mut verlet_data: Mut<'_, VerletData>, mut entity_pos: Mut<'_, bevy::prelude::Transform>)
pub fn apply_gravity(verlet_data: &mut VerletData, _entity_pos: &mut bevy::prelude::Transform)
{
    verlet_data.acceleration += verlet_data.base_gravity;
    // entity_pos.translation.y += verlet_data.base_gravity.y;  // Will move to UpdatePosition later
}

// pub fn apply_constraints(mut verlet_data: Mut<'_, VerletData>, mut entity_pos: Mut<'_, bevy::prelude::Transform>)
pub fn apply_constraints(verlet_data: &mut VerletData, entity_pos: &mut bevy::prelude::Transform)
{
    let circle_position = Vec2 {x: entity_pos.translation.x, y: entity_pos.translation.y};
    let pit_center = MY_PIT.pit_center;
    let vector_to_center = circle_position - pit_center;
    let dist_to_center = circle_position.distance(pit_center);

    if dist_to_center + BALL_RADIUS > MY_PIT.pit_radius {
        let new_vec = vector_to_center / dist_to_center;
        let new_pos = pit_center + new_vec * (MY_PIT.pit_radius - BALL_RADIUS);
        // println!("******* vector_to_center: {}, distance: {}, new_pos: {}", vector_to_center, dist_to_center, new_pos);

        verlet_data.pos_current.x = new_pos.x;
        verlet_data.pos_current.y = new_pos.y;
    }
}

pub fn update_position(verlet_data: &mut VerletData, entity_pos: &mut bevy::prelude::Transform)
{
    let velocity = verlet_data.pos_current - verlet_data.pos_old;
    verlet_data.pos_old = verlet_data.pos_current;

    // Here it is - the actual Verlet equation!
    verlet_data.pos_current = verlet_data.pos_current + velocity +
        verlet_data.acceleration * (verlet_data.delta_t * verlet_data.delta_t);

    // Copy the verlet-positioning, to the Entity transform - piecemeal, because transform is a Vec3
    entity_pos.translation.x = verlet_data.pos_current.x;
    entity_pos.translation.y = verlet_data.pos_current.y;

    verlet_data.acceleration = Vec2 { x: 0.0, y: 0.0 };
}
