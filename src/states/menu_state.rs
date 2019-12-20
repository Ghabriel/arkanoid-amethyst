use amethyst::{
    assets::Loader,
    core::ArcThreadPool,
    ecs::prelude::*,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::{
    audio::initialise_audio,
    config::GameConfig,
    states::{AboutState, GameState},
    systems::MenuSystem,
};

use std::ops::Deref;

pub struct Menu {
    pub focused_item: usize,
    pub selected: bool,
    pub items: Vec<Entity>,
}

fn initialise_menu(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let (new_game_transform, about_transform) = {
        let config = world.read_resource::<GameConfig>();

        let new_game_transform = UiTransform::new(
            "New Game".to_string(),
            Anchor::TopMiddle, Anchor::TopMiddle,
            0., 50.0 - config.arena.height / 3.0, 1.,
            400., 100.,
        );

        let about_transform = UiTransform::new(
            "About".to_string(),
            Anchor::TopMiddle, Anchor::TopMiddle,
            0., 50.0 - 2.0 * config.arena.height / 3.0, 1.,
            400., 100.,
        );

        (new_game_transform, about_transform)
    };

    let new_game_text = UiText::new(
        font.clone(),
        "New Game".to_string(),
        [1., 1., 1., 1.],
        50.,
    );

    let about_text = UiText::new(
        font.clone(),
        "About".to_string(),
        [1., 1., 1., 0.01],
        50.,
    );

    let new_game = world
        .create_entity()
        .with(new_game_transform)
        .with(new_game_text)
        .build();

    let about = world
        .create_entity()
        .with(about_transform)
        .with(about_text)
        .build();

    world.insert(Menu {
        focused_item: 0,
        selected: false,
        items: vec![new_game, about],
    });
}

#[derive(Default)]
pub struct MenuState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl SimpleState for MenuState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(MenuSystem::new(data.world), "menu_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone())
            .build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);

        initialise_audio(data.world);
        initialise_menu(data.world);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let menu_items = &world.read_resource::<Menu>().items.clone();
        world.delete_entities(menu_items).expect("Failed to delete menu items");
        world.remove::<Menu>();
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_menu(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        let menu = data.world.read_resource::<Menu>();
        if menu.selected {
            match menu.focused_item {
                0 => Trans::Switch(Box::new(GameState::default())),
                1 => Trans::Push(Box::new(AboutState::default())),
                _ => unreachable!(),
            }
        } else {
            Trans::None
        }
    }
}
