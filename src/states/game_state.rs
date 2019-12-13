use amethyst::{
    assets::{
        AssetStorage,
        Handle,
        Loader,
        Prefab,
        PrefabLoader,
        ProgressCounter,
        RonFormat,
    },
    core::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat},
};

use crate::{
    config::GameConfig,
    entities::ball::{Ball, initialise_ball},
    entities::brick::{Brick, BrickPrefab},
    entities::paddle::{Paddle, initialise_paddle},
};

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

pub struct GameState {
    pub progress_counter: ProgressCounter,
    pub prefab_handle: Option<Handle<Prefab<BrickPrefab>>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            progress_counter: ProgressCounter::new(),
            prefab_handle: None,
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let sprite_sheet = load_sprite_sheet(&world);

        world.register::<Ball>();
        world.register::<Brick>();
        world.register::<Paddle>();
        initialise_ball(&mut world, sprite_sheet.clone());
        initialise_paddle(&mut world, sprite_sheet.clone());
        initialise_camera(&mut world);

        let prefab_handle = world.exec(|loader: PrefabLoader<'_, BrickPrefab>| {
            loader.load(
                "prefabs/level1.ron",
                RonFormat,
                &mut self.progress_counter,
            )
        });

        world
            .create_entity()
            .with(prefab_handle.clone())
            .build();

        self.prefab_handle = Some(prefab_handle);
    }
}
