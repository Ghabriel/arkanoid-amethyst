use amethyst::{
    assets::{
        Handle,
        Loader,
        Prefab,
        PrefabLoader,
        ProgressCounter,
        RonFormat,
    },
    core::Transform,
    ecs::{
        prelude::*,
        world::EntitiesRes,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat},
};

use crate::{
    config::GameConfig,
    entities::ball::{Ball, initialise_ball},
    entities::brick::{Brick, BrickPrefab},
    entities::paddle::{Paddle, initialise_paddle},
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

#[derive(Clone)]
pub enum LevelState {
    Cleared,
    GameOver,
    Playing,
}

pub struct GameState {
    pub sprite_sheet: Option<Handle<SpriteSheet>>,
    pub progress_counter: ProgressCounter,
    pub prefab_handle: Option<Handle<Prefab<BrickPrefab>>>,
    pub attached_sprites_to_bricks: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            sprite_sheet: None,
            progress_counter: ProgressCounter::new(),
            prefab_handle: None,
            attached_sprites_to_bricks: false,
        }
    }

    fn load_level(&mut self, world: &mut World, level: LevelId) {
        if self.sprite_sheet.is_none() {
            self.sprite_sheet = Some(load_sprite_sheet(&world));
        }

        world.delete_all();

        let sprite_sheet = self.sprite_sheet.as_ref().unwrap();
        initialise_ball(world, sprite_sheet.clone());
        initialise_paddle(world, sprite_sheet.clone());
        initialise_camera(world);

        let prefab_handle = world.exec(|loader: PrefabLoader<'_, BrickPrefab>| {
            loader.load(
                format!("prefabs/level{}.ron", level),
                RonFormat,
                &mut self.progress_counter,
            )
        });

        world.insert(LevelData {
            current_level: level,
            state: LevelState::Playing,
        });

        world
            .create_entity()
            .with(prefab_handle.clone())
            .build();

        self.prefab_handle = Some(prefab_handle);
        self.attached_sprites_to_bricks = false;
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.load_level(data.world, 1);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
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
        }

        let level_data = data.world.read_resource::<LevelData>().deref().clone();
        match level_data.state {
            LevelState::Cleared => {
                println!("Level Cleared");
            },
            LevelState::GameOver => self.load_level(&mut data.world, level_data.current_level),
            LevelState::Playing => {},
        }

        Trans::None
    }
}
