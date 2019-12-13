use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    config::GameConfig,
    entities::paddle::Paddle,
};

pub struct PaddleMovementSystem;

impl<'a> System<'a> for PaddleMovementSystem {
    type SystemData = (
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, GameConfig>,
    );

    fn run(&mut self, (paddles, mut transforms, input, config): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("paddle").unwrap_or(0.0);
            let scaled_movement = 2.0 * movement;
            let paddle_x = transform.translation().x;

            transform.set_translation_x(
                (paddle_x + scaled_movement)
                    .min(config.arena.width - config.paddle.width / 2.0)
                    .max(config.paddle.width / 2.0)
            );
        }
    }
}
