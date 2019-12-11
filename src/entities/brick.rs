use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

pub struct Brick {
    pub width: f32,
    pub height: f32,
}

impl Component for Brick {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_bricks(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let brick = Brick {
        width: 50.0,
        height: 20.0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(80.0, 550.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 2,
    };

    world
        .create_entity()
        .with(brick)
        .with(transform)
        .with(sprite_render)
        .build();
}
