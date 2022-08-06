/*

*/

use bevy::{prelude::*};
use bevy::math::Vec2;

#[derive(Component, Debug)]
pub struct VerletData {
    pub pos_current: Vec2,
    pub pos_old: Vec2
}
