use amethyst::{
    ecs::{
        ReadStorage,
        System,
        WriteExpect,
    },
};

use crate::{
    entities::brick::Brick,
    states::game_state::{LevelData, LevelState},
};

pub struct LevelClearSystem;

impl<'a> System<'a> for LevelClearSystem {
    type SystemData = (
        ReadStorage<'a, Brick>,
        WriteExpect<'a, LevelData>,
    );

    fn run(&mut self, (bricks, mut level_data): Self::SystemData) {
        if level_data.state == LevelState::Playing && bricks.is_empty() {
            level_data.state = LevelState::Cleared;
        }
    }
}
