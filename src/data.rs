use bevy::prelude::Color;
use bevy::math::Vec2;

pub(crate) const BALL_RADIUS: f32 = 40.0;

pub(crate) struct BallPit {
    pub pit_center: Vec2,
    pub pit_radius: f32
}

pub(crate) const MY_PIT: BallPit = BallPit
    { pit_center: Vec2 { x: -475.0, y: 0.0 },
      pit_radius: 425.0 };

// Assorted screen data
pub const HEIGHT: f32 = 950.0;
pub const WIDTH: f32 = 1870.0;
// const ASPECT_RATIO: f32 = WIDTH / HEIGHT;
pub const BACKGROUND: Color = Color::rgb(0.50, 0.50, 1.0); // Purple

pub const BLOCK_SIZE: f32 = 50.0;
pub const PADDLE_COLOR: Color = Color::rgba(0.3, 0.1, 0.9, 0.9); // Green
