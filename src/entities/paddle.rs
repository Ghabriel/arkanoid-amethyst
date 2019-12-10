use amethyst::{
    assets::{Handle, Loader},
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet},
};

pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_paddle(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    // let mut transform = Transform::default();

    // transform.set_translation_xyz(x: f32, y: f32, z: f32)

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity();
        // .with();
}
