/*
Note that Bevy's Y-axis goes down the screen - so, like Python.Kivy,
acceleration is reversed.
*/

use bevy::{prelude::*};
use bevy::math::Vec2;

use crate::data::*;
use crate::{GuiData, PitActive};

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {print!("{}", std::any::type_name::<T>())} // Unstable.

#[derive(Component, Debug)]
pub struct VerletData {
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub base_gravity: Vec2,
    pub acceleration: Vec2,
    pub delta_t: f32,
}

pub fn solve_for_verlet(action_status: Res<PitActive>,
                        random_data: Res<GuiData>,
                        mut balls_qry: Query<(Entity, &mut VerletData, &mut Transform)>,
){
    if action_status.is_paused { return; }
    if action_status.game_status != GameState::Running { return; }

    let ball_radius = random_data.radius_slider_value;

    for (_entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        // apply_gravity(verlet_data, entity_pos);  // No hope that this would ever work. See lessons.md
        apply_gravity(&mut verlet_data, &mut *entity_pos);
        apply_constraints(&mut verlet_data, &mut *entity_pos,ball_radius);
    }

    // I can't believe this works...
    let mut balls: Vec<(Entity, Mut<'_, VerletData>, Mut<'_, Transform>)> = balls_qry.iter_mut().collect();
    let ball_count = balls.len();

    for ball_one in 0..ball_count {
        for ball_two in 0..ball_count {
            if balls[ball_one].0 == balls[ball_two].0 { continue }
            let maybe_vec = check_for_collision(&balls[ball_one], &balls[ball_two], ball_radius);
            if maybe_vec == None {continue};
            let new_vec = maybe_vec.unwrap();

            balls[ball_one].1.pos_current += new_vec;
            balls[ball_two].1.pos_current -= new_vec;
        }
    }

    for (_entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        update_position(&mut verlet_data, &mut *entity_pos);
    }
}

fn check_for_collision(ball_one: &(Entity, Mut<VerletData>, Mut<Transform>),
                       ball_two: &(Entity, Mut<VerletData>, Mut<Transform>),
                       ball_radius: f32,
) -> Option<Vec2> {
    //println!("I have no idea what this is: {:?}", ball_one.1);
    //println!("But here is another one: {:?}", ball_two.1);

    let collision_axis = ball_one.1.pos_current - ball_two.1.pos_current;
    let collision_distance = ball_one.1.pos_current.distance(ball_two.1.pos_current);

    if collision_distance < (ball_radius * 2.0) {
        let new_vec = collision_axis / collision_distance;
        let new_delta = (ball_radius * 2.0) - collision_distance;
        let new_pos = new_vec * new_delta * DAMP_FACTOR;
        return Some(new_pos);

    }
    return None;
}

// pub fn apply_gravity(mut verlet_data: Mut<'_, VerletData>, mut entity_pos: Mut<'_, bevy::prelude::Transform>)
fn apply_gravity(verlet_data: &mut VerletData, _entity_pos: &mut Transform)
{
    verlet_data.acceleration += verlet_data.base_gravity;
}

// pub fn apply_constraints(mut verlet_data: Mut<'_, VerletData>, mut entity_pos: Mut<'_, bevy::prelude::Transform>)
fn apply_constraints(verlet_data: &mut VerletData, entity_pos: &mut Transform,
                         ball_radius: f32,
) {
    let circle_position = Vec2 {x: entity_pos.translation.x, y: entity_pos.translation.y};
    let pit_center = MY_PIT.pit_center;
    let vector_to_center = circle_position - pit_center;
    let dist_to_center = circle_position.distance(pit_center);

    if dist_to_center + ball_radius > MY_PIT.pit_radius {
        let new_vec = vector_to_center / dist_to_center;
        let new_pos = pit_center + new_vec * (MY_PIT.pit_radius - ball_radius);

        verlet_data.pos_current.x = new_pos.x;
        verlet_data.pos_current.y = new_pos.y;
    }
}

pub fn update_position(verlet_data: &mut VerletData, entity_pos: &mut Transform)
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
