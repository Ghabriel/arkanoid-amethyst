use amethyst::{
    assets::{
        Handle,
        Loader,
        Prefab,
        PrefabLoader,
        ProgressCounter,
        RonFormat,
    },
    core::{ArcThreadPool, Transform},
    ecs::prelude::*,
    input::InputEvent,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat},
};

use crate::{
    config::GameConfig,
    entities::{
        ball::initialise_ball,
        brick::{Brick, BrickPrefab},
        paddle::initialise_paddle,
    },
    states::PauseState,
    systems::{
        BallBounceSystem,
        BallMovementSystem,
        LevelClearSystem,
        PaddleMovementSystem,
    },
};

use std::ops::Deref;

pub fn initialise_camera(world: &mut World) {
    let (arena_width, arena_height) = {
        let config = &world.read_resource::<GameConfig>();
        (config.arena.width, config.arena.height)
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(arena_width / 2.0, arena_height / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(arena_width, arena_height))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = loader.load(
        "texture/arkanoid.png",
        ImageFormat::default(),
        (),
        &world.read_resource(),
    );

    loader.load(
        "texture/arkanoid.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource(),
    )
}

pub type LevelId = u8;

#[derive(Clone)]
pub struct LevelData {
    pub current_level: LevelId,
    pub state: LevelState,
}

#[derive(Clone, Eq, PartialEq)]
pub enum LevelState {
    Cleared,
    GameOver,
    Loading,
    Playing,
}

#[derive(Default)]
pub struct GameState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub sprite_sheet: Option<Handle<SpriteSheet>>,
    pub progress_counter: ProgressCounter,
    pub prefab_handle: Option<Handle<Prefab<BrickPrefab>>>,
    pub attached_sprites_to_bricks: bool,
}

impl GameState<'_, '_> {
    fn load_level(&mut self, world: &mut World, level: LevelId) {
        if self.sprite_sheet.is_none() {
            self.sprite_sheet = Some(load_sprite_sheet(&world));
        }

        world.delete_all();

        let sprite_sheet = self.sprite_sheet.as_ref().unwrap();
        initialise_ball(world, sprite_sheet.clone());
        initialise_paddle(world, sprite_sheet.clone());
        initialise_camera(world);

        world.insert(LevelData {
            current_level: level,
            state: LevelState::Loading,
        });

        let prefab_handle = world.exec(|loader: PrefabLoader<'_, BrickPrefab>| {
            loader.load(
                format!("prefabs/level{}.ron", level),
                RonFormat,
                &mut self.progress_counter,
            )
        });

        world
            .create_entity()
            .with(prefab_handle.clone())
            .build();

        self.prefab_handle = Some(prefab_handle);
        self.attached_sprites_to_bricks = false;
    }
}

impl SimpleState for GameState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(BallMovementSystem, "ball_movement_system", &[]);
        dispatcher_builder.add(BallBounceSystem, "ball_bounce_system", &["ball_movement_system"]);
        dispatcher_builder.add(LevelClearSystem, "level_clear_system", &["ball_bounce_system"]);
        dispatcher_builder.add(PaddleMovementSystem, "paddle_movement_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone())
            .build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);

        self.load_level(data.world, 1);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        if self.progress_counter.is_complete() && !self.attached_sprites_to_bricks {
            data.world.exec(
                |(entities, bricks, mut sprite_renders): (
                    Entities,
                    ReadStorage<Brick>,
                    WriteStorage<SpriteRender>,
                )| {
                    for (entity, _) in (&entities, &bricks).join() {
                        let sprite_render = SpriteRender {
                            sprite_sheet: self.sprite_sheet.clone().unwrap(),
                            sprite_number: 2,
                        };

                        sprite_renders
                            .insert(entity, sprite_render)
                            .unwrap();
                    }

                    self.attached_sprites_to_bricks = true;
                },
            );

            data.world.write_resource::<LevelData>().state = LevelState::Playing;
        }

        let level_data = data.world.read_resource::<LevelData>().deref().clone();
        match level_data.state {
            LevelState::Cleared => self.load_level(&mut data.world, level_data.current_level + 1),
            LevelState::GameOver => self.load_level(&mut data.world, level_data.current_level),
            LevelState::Loading => {},
            LevelState::Playing => {},
        }

        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(
                InputEvent::ActionPressed(action)
            ) if action == "pause" => Trans::Push(Box::new(PauseState::default())),
            _ => Trans::None,
        }
    }
}
