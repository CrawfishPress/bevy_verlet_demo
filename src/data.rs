/*
I set a default() function for PitActive - but the only default value
that I needed to change was `is_paused`. I *could* just toggle it to
`is_running`, then wouldn't need the default code-block - but it would
change the semantics, and I tend to think that:
    if action_status.is_paused { return; }
is slightly more clear than
    if ! action_status.is_running { return; }
*/

use bevy::prelude::*;
use bevy::math::Vec2;

pub const CIRCLE_DELAY: f32 = 0.5;
pub const DAMP_FACTOR: f32 = 0.5;

// Assorted screen data
pub const HEIGHT: f32 = 950.0;
pub const WIDTH: f32 = 1870.0;
pub const BACKGROUND: Color = Color::rgb(0.50, 0.50, 1.0); // Purple

pub const BLOCK_SIZE: f32 = 50.0;
pub const PADDLE_COLOR: Color = Color::rgba(0.3, 0.1, 0.9, 0.9); // Green

// Resource - general Game status
pub struct PitActive {
    pub game_status: GameState,
    pub is_paused: bool,
    pub balls_added: i32,
    pub total_balls: i32,
}

impl Default for PitActive {
    fn default() -> Self {
        PitActive {
            game_status: GameState::NotStarted,
            is_paused: true,
            balls_added: 0,
            total_balls: 0
        }
    }
}

// Cycled by a GUI Reset button
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    NotStarted,
    Running,
    Reset,
}

// Adding a cycler-function to simplify changing states
impl GameState {
    pub fn cycle(&self) -> Self {
        match *self {
            GameState::NotStarted => GameState::Running,
            GameState::Running => GameState::Reset,
            GameState::Reset => GameState::NotStarted,
        }
    }
}

pub struct BallPit {
    pub pit_center: Vec2,
    pub pit_radius: f32
}

pub const MY_PIT: BallPit = BallPit {
      pit_center: Vec2 { x: -475.0, y: 0.0 },
      pit_radius: 425.0 };

// Marker for Circle sprites
#[derive(Component)]
pub struct OneCircle;

// GUI Resource
pub struct GuiData {
    pub some_name: String,
    pub total_balls: i32,
    pub radius_slider_value: f32,
    pub ball_slider_value: i32,
}

impl Default for GuiData {
    fn default() -> Self {
        GuiData {
            some_name: "".to_string(),
            total_balls: 0,
            radius_slider_value: 15.0,
            ball_slider_value: 20
        }
    }
}
