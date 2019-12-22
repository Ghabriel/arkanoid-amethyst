use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::GameConfig;

#[derive(Debug)]
pub struct Ball {
    pub radius: f32,
    pub velocity: [f32; 2],
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let (ball, transform, sprite_render) = {
        let config = world.read_resource::<GameConfig>();
        let (sin, cos) = config.ball.initial_angle
            .to_radians()
            .sin_cos();

        let ball = Ball {
            radius: config.ball.radius,
            velocity: [
                config.ball.initial_velocity * cos,
                config.ball.initial_velocity * sin,
            ],
        };

        let mut transform = Transform::default();
        let x = config.arena.width / 2.0;
        let y = config.paddle.margin + config.paddle.height + config.ball.radius;
        transform.set_translation_xyz(x, y, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 1,
        };

        (ball, transform, sprite_render)
    };

    world
        .create_entity()
        .with(ball)
        .with(transform)
        .with(sprite_render)
        .build()
}

pub fn change_speed<F>(ball: &mut Ball, change: F)
where
    F: Fn(f32) -> f32,
{
    let ball_vx = ball.velocity[0];
    let ball_vy = ball.velocity[1];
    let angle = ball_vy.atan2(ball_vx);
    let (sin, cos) = angle.sin_cos();
    let old_velocity = ball_vx.hypot(ball_vy);
    let new_velocity = change(old_velocity);
    ball.velocity[0] = new_velocity * cos;
    ball.velocity[1] = new_velocity * sin;
}

pub fn change_direction<F>(ball: &mut Ball, change: F)
where
    F: Fn(f32) -> f32,
{
    let ball_vx = ball.velocity[0];
    let ball_vy = ball.velocity[1];
    let angle = change(ball_vy.atan2(ball_vx));
    let (sin, cos) = angle.sin_cos();
    let ball_velocity = ball_vx.hypot(ball_vy);
    ball.velocity[0] = ball_velocity * cos;
    ball.velocity[1] = ball_velocity * sin;
}
