use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    // prelude::*,
};

use crate::entities::ball::Ball;

pub struct BallMovementSystem;

impl<'a> System<'a> for BallMovementSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
