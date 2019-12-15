use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::GameConfig;

#[derive(Debug)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_paddle(
    world: &mut World,
    sprite_sheet: Handle<SpriteSheet>,
) {
    let (paddle, transform, sprite_render) = {
        let config = &world.read_resource::<GameConfig>();

        let paddle = Paddle {
            width: config.paddle.width,
            height: config.paddle.height,
        };

        let mut transform = Transform::default();
        let x = config.arena.width / 2.0;
        let y = config.paddle.height / 2.0 + config.paddle.margin;
        transform.set_translation_xyz(x, y, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        (paddle, transform, sprite_render)
    };

    world
        .create_entity()
        .with(paddle)
        .with(transform)
        .with(sprite_render)
        .build();
}
