/*
Note that Bevy's Y-axis goes down the screen - so, like Python.Kivy,
acceleration is reversed.
*/

use bevy::{prelude::*};
use bevy::math::Vec2;

#[derive(Component, Debug)]
pub struct VerletData {
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub base_gravity: Vec2,
}
