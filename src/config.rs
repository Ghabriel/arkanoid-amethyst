use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
    pub use_joystick_keybindings: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            width: 800.0,
            height: 600.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BallConfig {
    pub initial_angle: f32,
    pub initial_velocity: f32,
    pub radius: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            initial_angle: 30.0,
            initial_velocity: 255.0,
            radius: 12.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
    // Distance from the border of the arena to the paddle
    pub margin: f32,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            width: 170.0,
            height: 12.0,
            margin: 15.0,
        }
    }
}
