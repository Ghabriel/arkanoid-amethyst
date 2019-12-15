mod config;
mod entities;
mod states;
mod systems;

use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::{
    config::GameConfig,
    entities::brick::BrickPrefab,
    states::GameState,
    systems::{BallBounceSystem, BallMovementSystem, PaddleMovementSystem},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");
    let keybindings_config_path = app_root.join("config").join("keybindings.ron");

    let game_config = GameConfig::load(
        app_root.join("config").join("game.ron")
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(keybindings_config_path)?
        )?
        .with_system_desc(
            PrefabLoaderSystemDesc::<BrickPrefab>::default(),
            "prefab_loader",
            &[],
        )
        .with(BallMovementSystem, "ball_movement_system", &[])
        .with(BallBounceSystem, "ball_bounce_system", &["ball_movement_system"])
        .with(PaddleMovementSystem, "paddle_movement_system", &["input_system"]);

    let assets_path = app_root.join("assets");
    Application::build(assets_path, GameState::new())?
        .with_resource(game_config)
        .build(game_data)?
        .run();

    Ok(())
}
