use amethyst::{
    assets::Loader,
    ecs::Entity,
    input::InputEvent,
    prelude::*,
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
};

use crate::{
    config::{ArenaConfig, GameConfig},
};

const MAIN_TEXT: &'static str = "\
Arkanoid
Created by Ghabriel Nunes
Made using Amethyst Engine

Start Date: 2019-12-10
";

const QUIT_TEXT: &'static str = "Press any key to return...";

#[derive(Default)]
pub struct AboutState {
    entities: Vec<Entity>,
}

impl SimpleState for AboutState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.entities = initialise_text(data.world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world
            .delete_entities(&self.entities)
            .expect("Failed to delete AboutState entities");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(InputEvent::ActionPressed { .. }) => Trans::Pop,
            _ => Trans::None,
        }
    }
}

fn initialise_text(world: &mut World) -> Vec<Entity> {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let arena_config = world.read_resource::<GameConfig>()
        .arena
        .clone();

    let main_text_entity = initialise_main_text(world, &font, &arena_config);
    let quit_text_entity = initialise_quit_text(world, &font, &arena_config);

    vec![main_text_entity, quit_text_entity]
}

fn initialise_main_text(world: &mut World, font: &FontHandle, arena_config: &ArenaConfig) -> Entity {
    let main_text_transform = UiTransform::new(
        "About - Main Text".to_string(),
        Anchor::TopMiddle, Anchor::TopMiddle,
        0., 0., 1.,
        arena_config.width, arena_config.height,
    );

    let mut main_text = UiText::new(
        font.clone(),
        MAIN_TEXT.to_string(),
        [1., 1., 1., 1.],
        30.,
    );
    main_text.line_mode = LineMode::Wrap;

    world
        .create_entity()
        .with(main_text_transform)
        .with(main_text)
        .build()
}

fn initialise_quit_text(world: &mut World, font: &FontHandle, arena_config: &ArenaConfig) -> Entity {
    let quit_text_transform = UiTransform::new(
        "About - Quit Text".to_string(),
        Anchor::BottomMiddle, Anchor::BottomMiddle,
        0., 0., 1.,
        arena_config.width, 30.,
    );

    let quit_text = UiText::new(
        font.clone(),
        QUIT_TEXT.to_string(),
        [1., 1., 1., 1.],
        30.,
    );

    world
        .create_entity()
        .with(quit_text_transform)
        .with(quit_text)
        .build()
}
