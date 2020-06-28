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

            let updown = input.axis_value("updown");
            let leftright = input.axis_value("leftright");

            if let Some(mv_amount) = updown {
                let new_position = Position {
                    x: occupant.position.x,
                    y: occupant.position.y + mv_amount as i8,
                };
                if !position_is_obstructed(&new_position) {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    occupant.position.y = new_position.y;
                    transform.prepend_translation_y(scaled_amount);
                }
            }

            if let Some(mv_amount) = leftright {
                let new_position = Position {
                    x: occupant.position.x + mv_amount as i8,
                    y: occupant.position.y,
                };
                if !position_is_obstructed(&new_position) {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    occupant.position.x = new_position.x;
                    transform.prepend_translation_x(scaled_amount);
                }
            }
        }
    }
}
