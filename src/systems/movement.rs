use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::simple_game::{
    TileOccupant,
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    TILE_ROWS,
    TILE_COLUMNS,
};

#[derive(SystemDesc)]
pub struct MovementSystem;


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
                let new_y_position = occupant.position.y + mv_amount as i8;
                if new_y_position >= 0 && new_y_position < TILE_ROWS as i8 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    occupant.position.y = new_y_position;
                    transform.prepend_translation_y(scaled_amount);
                }

            }

            if let Some(mv_amount) = leftright {
                let new_x_position = occupant.position.x + mv_amount as i8;
                if new_x_position >= 0 && new_x_position < TILE_COLUMNS as i8 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    occupant.position.x = new_x_position;
                    transform.prepend_translation_x(scaled_amount);
                }
            }
        }
    }
}
