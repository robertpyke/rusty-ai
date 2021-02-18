use specs::prelude::*;

use crate::components::*;

pub struct EnemyOOBPurger;

// TODO Get these from world (our a global config)
const MIN_X: i32 = -300;
const MAX_X: i32 = 300;
const MIN_Y: i32 = -300;
const MAX_Y: i32 = 300;

/**
 * Purge all enemies who leave the game area.
 */
impl<'a> System<'a> for EnemyOOBPurger {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Enemy>,
    );

    fn run(&mut self, (entities, positions, enemies): Self::SystemData) {
        for (entity, pos, _) in (&entities, &positions, &enemies).join() {
            if pos.0.x < MIN_X || pos.0.x > MAX_X || pos.0.y < MIN_Y || pos.0.y > MAX_Y {
                entities.delete(entity).unwrap();
            }
        }
    }
}
