mod config;
mod entities;
mod states;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::{
    config::GameConfig,
    states::GameState,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");

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
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let assets_path = app_root.join("assets");
    Application::build(assets_path, GameState)?
        .with_resource(game_config)
        .build(game_data)?
        .run();

    Ok(())
}
