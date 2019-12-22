use amethyst::{
    ecs::{ReadStorage, System, WriteExpect},
};

use crate::{
    entities::ball::Ball,
    states::game_state::{LevelData, LevelState},
};

pub struct GameoverSystem;

impl<'a> System<'a> for GameoverSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        WriteExpect<'a, LevelData>,
    );

    fn run(&mut self, (balls, mut level_data): Self::SystemData) {
        if balls.is_empty() {
            level_data.state = LevelState::GameOver;
        }
    }
}
