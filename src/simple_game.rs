use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const DISPLAY_HEIGHT: f32 = 64.0;
pub const DISPLAY_WIDTH: f32 = 64.0;

pub const TILE_HEIGHT: f32 = 16.0;
pub const TILE_WIDTH: f32 = 16.0;

pub enum TileContent {
   Character, 
   Wall,
   Floor,
}

pub struct Tile {
    pub content: TileContent,
}

// By implementing Component, Tile can
// now be attached to entities in the game
impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}


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
    transform.set_translation_xyz(DISPLAY_WIDTH * 0.5, DISPLAY_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .with(transform)
        .build();
}

