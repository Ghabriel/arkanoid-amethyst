use amethyst::{
    assets::Loader,
    audio::{AudioSink, DjSystem, output::{init_output, Output}},
    core::ArcThreadPool,
    ecs::prelude::*,
    input::InputEvent,
    prelude::*,
    shrev::EventChannel,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::{
    audio::{initialise_audio, Music},
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

#[derive(Debug)]
pub struct MenuEvent {
    pub action_pressed: String,
}

#[derive(Default)]
pub struct MenuState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl SimpleState for MenuState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(
                DjSystem::new(|music: &mut Music| Some(music.opening.clone())),
                "dj_system",
                &[],
            )
            .with(MenuSystem::default(), "menu_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        init_output(world);
        world.write_resource::<AudioSink>().set_volume(0.25);
        initialise_audio(world);
        initialise_menu(world);

        world.insert(EventChannel::<MenuEvent>::new());
        let reader = world.write_resource::<EventChannel<MenuEvent>>().register_reader();
        world.insert(reader);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.remove::<AudioSink>();
        data.world.remove::<Output>();
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let menu_items = &world.read_resource::<Menu>().items.clone();
        world.delete_entities(menu_items).expect("Failed to delete menu items");
        world.remove::<Menu>();
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_menu(data.world);
        let reader = data.world.fetch_mut::<EventChannel<MenuEvent>>().register_reader();
        data.world.insert(reader);
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

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(InputEvent::ActionPressed(action)) => {
                data.world
                    .write_resource::<EventChannel<MenuEvent>>()
                    .single_write(MenuEvent {
                        action_pressed: action,
                    });
            },
            _ => {},
        }

        Trans::None
    }
}
