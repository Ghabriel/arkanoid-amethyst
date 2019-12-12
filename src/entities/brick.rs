use amethyst::{
    assets::{Handle, PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PrefabData)]
pub struct BrickPrefab {
    brick: Brick,
    transform: Transform,
    // sprite_render: SpriteRender,
}

#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
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
