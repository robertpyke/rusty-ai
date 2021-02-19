use specs::prelude::*;

use crate::components::*;

pub struct EnemyOOBPurger;

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
        let min_x: i32 = -(super::WORLD_WIDTH as i32 / 2);
        let max_x: i32 = super::WORLD_WIDTH as i32 / 2;
        let min_y: i32 = -(super::WORLD_HEIGHT as i32 / 2);
        let max_y: i32 = super::WORLD_HEIGHT as i32 / 2;

        for (entity, pos, _) in (&entities, &positions, &enemies).join() {
            if pos.0.x < min_x || pos.0.x > max_x || pos.0.y < min_y || pos.0.y > max_y {
                entities.delete(entity).unwrap();
            }
        }
    }
}
