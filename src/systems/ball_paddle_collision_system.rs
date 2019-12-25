use amethyst::{
    core::Transform,
    ecs::{
        Join,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::{
    audio::{Sound, SoundKit},
    entities::{
        ball::Ball,
        paddle::Paddle,
    },
};

pub struct BallPaddleCollisionSystem;

impl<'a> System<'a> for BallPaddleCollisionSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        SoundKit<'a>,
    );

    fn run(&mut self, (mut balls, transforms, paddles, sound_kit): Self::SystemData) {
        for (mut ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            for (paddle, transform) in (&paddles, &transforms).join() {
                let paddle_x = transform.translation().x;
                let paddle_y = transform.translation().y;

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - 0.5 * paddle.width - ball.radius,
                    paddle_y - 0.5 * paddle.height - ball.radius,
                    paddle_x + 0.5 * paddle.width + ball.radius,
                    paddle_y + 0.5 * paddle.height + ball.radius,
                ) {
                    if ball.velocity[1] < 0.0 {
                        sound_kit.play_sound(Sound::Bounce);

                        let ball_velocity = ball.velocity[0].hypot(ball.velocity[1]);
                        let dx = ball_x - paddle_x;
                        let dy = ball_y - (paddle_y - 5.0);
                        let angle = dy.atan2(dx);
                        let (sin, cos) = angle.sin_cos();
                        ball.velocity[0] = ball_velocity * cos;
                        ball.velocity[1] = ball_velocity * sin;
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
