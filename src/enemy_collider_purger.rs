use specs::prelude::*;

use crate::components::*;

pub struct EnemyColliderPurger;

use super::sprite;
use sdl2::rect::Rect;

/**
 * Purge all enemies who collide with a hero
 */
impl<'a> System<'a> for EnemyColliderPurger {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Hero>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Telemetry>,
    );

    fn run(&mut self, (entities, positions, heroes, enemies, mut telemetries): Self::SystemData) {
        for (hero_pos, _) in (&positions, &heroes).join() {
            let hero_rect = Rect::new(
                hero_pos.0.x,
                hero_pos.0.y,
                sprite::HERO_FRAME_WIDTH,
                sprite::HERO_FRAME_HEIGHT,
            );

            for (enemy_entity, enemy_pos, _) in (&entities, &positions, &enemies).join() {
                let enemy_rect = Rect::new(
                    enemy_pos.0.x,
                    enemy_pos.0.y,
                    sprite::ENEMY_FRAME_WIDTH,
                    sprite::ENEMY_FRAME_HEIGHT,
                );

                if hero_rect.intersection(enemy_rect).is_some() {
                    entities.delete(enemy_entity).unwrap();
                    match (&mut telemetries).join().last() {
                        Some(telemetry) => telemetry.enemy_collisions += 1,
                        None => eprintln!("Telemetry Missing"),
                    }
                }
            }
        }
    }
}
