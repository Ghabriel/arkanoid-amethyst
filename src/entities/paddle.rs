use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        debug_drawing::DebugLinesComponent,
        palette::Srgba,
        SpriteRender,
        SpriteSheet,
    },
};

use crate::config::GameConfig;

pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_paddle(
    world: &mut World,
    // sprite_sheet: Handle<SpriteSheet>,
) {
    // let (paddle, transform, sprite_render) = {
    let (paddle, transform, debug_lines) = {
        let config = &world.read_resource::<GameConfig>();

        let paddle = Paddle {
            width: config.paddle.width,
            height: config.paddle.height,
        };

        let mut transform = Transform::default();
        let x = config.arena.width / 2.0;
        let y = config.paddle.height / 2.0 + config.paddle.margin;
        transform.set_translation_xyz(x, y, 0.0);

        // let sprite_render = SpriteRender {
        //     sprite_sheet,
        //     sprite_number: 0,
        // };

        // (paddle, transform, sprite_render)

        let mut debug_lines = DebugLinesComponent::new();
        debug_lines.add_rectangle_2d(
            [
                (config.arena.width - config.paddle.width) / 2.0,
                config.paddle.margin
            ].into(),
            [
                (config.arena.width + config.paddle.width) / 2.0,
                config.paddle.height + config.paddle.margin
            ].into(),
            0.0,
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        );

        (paddle, transform, debug_lines)
    };

    world
        .create_entity()
        .with(paddle)
        .with(transform)
        .with(debug_lines)
        // .with(sprite_render)
        .build();
}
