use amethyst::{
    assets::{Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat},
};

use crate::config::GameConfig;
use std::ops::Deref;

pub fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = loader.load(
        "texture/pong_spritesheet.png",
        ImageFormat::default(),
        (),
        &world.read_resource(),
    );

    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource(),
    )
}

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let config = &world.read_resource::<GameConfig>();
        println!("Started with config: {:#?}", config.deref());
    }
}
