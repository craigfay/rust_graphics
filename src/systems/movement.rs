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
        ReadStorage<'s, Tile>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, tiles, input): Self::SystemData) {
        // Iterating over all entities that have both a
        // tile component and a transform component
        for (tile, transform) in (&tiles, &mut transforms).join() {

            let updown = input.axis_value("updown");
            let leftright = input.axis_value("leftright");

            if let Some(mv_amount) = updown {
                if mv_amount != 0.0 {
                    println!("updown is {}", mv_amount);
                }
            }

            if let Some(mv_amount) = leftright {
                if mv_amount != 0.0 {
                    println!("leftright is {}", mv_amount);
                }
            }
        }
    }
}
