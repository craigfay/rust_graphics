use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::simple_game::{Tile, TileContent, DISPLAY_WIDTH, DISPLAY_HEIGHT};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    // Defining the data that the system
    // will operate on
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tile>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut tiles, input): Self::SystemData) {
        // Iterating over all entities that have both a
        // tile component and a transform component
        for (tile, transform) in (&mut tiles, &mut transforms).join() {

            let updown = input.axis_value("updown");
            let leftright = input.axis_value("leftright");

            if let Some(mv_amount) = updown {
                tile.is_actionable = false;
                let scaled_amount = 1.2 * mv_amount as f32;
                transform.prepend_translation_y(scaled_amount);
            }

            if let Some(mv_amount) = leftright {
                tile.is_actionable = false;
                let scaled_amount = 1.2 * mv_amount as f32;
                transform.prepend_translation_x(scaled_amount);
            }
        }
    }
}
