use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};


pub const TILE_HEIGHT: f32 = 16.0;
pub const TILE_WIDTH: f32 = 16.0;

pub const TILE_ROWS: f32 = 16.0;
pub const TILE_COLUMNS: f32 = 16.0;

pub const DISPLAY_HEIGHT: f32 = TILE_ROWS * TILE_HEIGHT;
pub const DISPLAY_WIDTH: f32 = TILE_COLUMNS * TILE_WIDTH;



pub enum TileContent {
   Character, 
   Wall,
   Floor,
}

pub struct Tile {
    pub content: TileContent,
    pub is_actionable: bool,
}

impl Tile {
    pub fn main_character() -> Tile {
        Tile {
            content: TileContent::Character,
            is_actionable: true,
        }
    }
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

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        // Register and initialize tile components
        world.register::<Tile>();
        initialize_tiles(world, sprite_sheet_handle);
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

fn initialize_tiles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut tile_1_transform = Transform::default();

    // Position tile 1
    let tile_1_y = DISPLAY_HEIGHT / 2.0;
    let tile_1_x = DISPLAY_WIDTH / 2.0;
    tile_1_transform.set_translation_xyz(tile_1_x, tile_1_y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    world
        .create_entity()
        .with(Tile::main_character())
        .with(tile_1_transform)
        .with(sprite_render)
        .build();
} 

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/overworld_character_tiles.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/overworld_character_tiles.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
