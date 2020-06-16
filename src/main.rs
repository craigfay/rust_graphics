use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


use amethyst::input::{InputBundle, StringBindings};
use amethyst::core::transform::TransformBundle;


mod simple_game;
use crate::simple_game::SimpleGame;

mod systems;
use crate::systems::MovementSystem;

fn main() -> amethyst::Result<()> {

    // Logging
    amethyst::start_logger(Default::default());

    // Display / Asset settings
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");



    // Input bindings
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    // Initialize game data
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides
                // all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities
                // with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(MovementSystem, "movement_system", &["input_system"]);

    // Start the game loop
    let mut game = Application::new(assets_dir, SimpleGame, game_data)?;
    game.run();
    
    Ok(())
}

