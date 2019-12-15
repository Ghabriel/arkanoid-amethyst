use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
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

pub fn initialise_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let (ball, transform, sprite_render) = {
        let config = world.read_resource::<GameConfig>();

        let ball = Ball {
            radius: config.ball.radius,
            velocity: [220.0, 130.0],
            // velocity: [120.0, 40.0],
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
        .build();
}
