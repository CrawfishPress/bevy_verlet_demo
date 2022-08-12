use bevy::prelude::Color;
use bevy::math::Vec2;

pub const CIRCLE_DELAY: f32 = 0.5;
pub const BALLS_MAX: i32 = 100;
pub const DAMP_FACTOR: f32 = 0.5;

// Assorted screen data
pub const HEIGHT: f32 = 950.0;
pub const WIDTH: f32 = 1870.0;
pub const BACKGROUND: Color = Color::rgb(0.50, 0.50, 1.0); // Purple

pub const BLOCK_SIZE: f32 = 50.0;
pub const PADDLE_COLOR: Color = Color::rgba(0.3, 0.1, 0.9, 0.9); // Green

// Resource
#[derive(PartialEq, PartialOrd)]
pub struct BallsInGame {
    pub balls_added: i32,
    pub total_balls: i32,
}

// Resource
pub struct PitActive {
    pub is_paused: bool,
}

pub(crate) struct BallPit {
    pub pit_center: Vec2,
    pub pit_radius: f32
}

pub(crate) const MY_PIT: BallPit = BallPit
    { pit_center: Vec2 { x: -475.0, y: 0.0 },
      pit_radius: 425.0 };

// GUI Resource
#[derive(Debug, Default)]
pub struct GuiData {
    pub some_name: String,
    pub total_balls: i32,
    pub radius_slider_value: f32,
    pub ball_slider_value: i32,
}
