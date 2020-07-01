use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::simple_game::{
    TileOccupant,
    Position,
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    TILE_ROWS,
    TILE_COLUMNS,
    TILE_SIZE,
};

#[derive(SystemDesc)]
pub struct MovementSystem;

fn position_is_obstructed(p: &Position) -> bool {
    if p.y < 0 || p.y >= TILE_ROWS as i8 { return true }
    if p.x < 0 || p.x >= TILE_COLUMNS as i8 { return true }
    false
}

impl<'s> System<'s> for MovementSystem {
    // Defining the data that the system
    // will operate on
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, TileOccupant>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut occupants, input): Self::SystemData) {
        // Iterating over all entities that have both a
        // tile component and a transform component
        for (occupant, transform) in (&mut occupants, &mut transforms).join() {

            let vertical = input.axis_value("y");
            let horizontal = input.axis_value("x");

            if let Some(mv_amount) = vertical{
                if mv_amount != 0.0 {
                    let new_position = Position {
                        x: occupant.position.x,
                        y: occupant.position.y + mv_amount as i8,
                    };
                    if !position_is_obstructed(&new_position) {
                        let y_position_in_pixels = TILE_SIZE
                            * mv_amount as f32;
                    
                        occupant.position.y = new_position.y;
                        transform.prepend_translation_y(y_position_in_pixels);
                    }
                }
            }

            if let Some(mv_amount) = horizontal {
                if mv_amount != 0.0 {
                    let new_position = Position {
                        x: occupant.position.x + mv_amount as i8,
                        y: occupant.position.y,
                    };
                    if !position_is_obstructed(&new_position) {
                        let x_position_in_pixels = TILE_SIZE
                            * mv_amount as f32;
                    
                        occupant.position.x = new_position.x;
                        transform.prepend_translation_x(x_position_in_pixels);
                    }
                }
            }
        }
    }
}
