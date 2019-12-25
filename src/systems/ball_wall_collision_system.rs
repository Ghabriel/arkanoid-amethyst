use amethyst::{
    core::Transform,
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::{
    audio::{Sound, SoundKit},
    config::GameConfig,
    entities::ball::Ball,
};

pub struct BallWallCollisionSystem;

impl<'a> System<'a> for BallWallCollisionSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        Entities<'a>,
        Read<'a, GameConfig>,
        SoundKit<'a>,
    );

    fn run(&mut self, (mut balls, transforms, entities, config, sound_kit): Self::SystemData) {
        for (ball_entity, mut ball, transform) in (&entities, &mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if (ball_x <= ball.radius && ball.velocity[0] < 0.0)
                || (ball_x >= config.arena.width - ball.radius && ball.velocity[0] > 0.0)
            {
                sound_kit.play_sound(Sound::Bounce);
                ball.velocity[0] = -ball.velocity[0];
            }

            if ball_y >= config.arena.height - ball.radius && ball.velocity[1] > 0.0 {
                sound_kit.play_sound(Sound::Bounce);
                ball.velocity[1] = -ball.velocity[1];
            }

            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                sound_kit.play_sound(Sound::GameOver);
                entities.delete(ball_entity).expect("Failed to delete ball");
            }
        }
    }
}
