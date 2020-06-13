use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const VIEW_HEIGHT: f32 = 100.0;
pub const VIEW_WIDTH: f32 = 100.0;

pub struct SimpleGame;

impl SimpleState for SimpleGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
    }
}

// Setup camera in a way that our screen
// covers whole arena and (0, 0) is in the bottom left.
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VIEW_WIDTH * 0.5, VIEW_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(VIEW_WIDTH, VIEW_HEIGHT))
        .with(transform)
        .build();
}

