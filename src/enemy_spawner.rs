use specs::prelude::*;

use crate::components::*;
use rand::prelude::*;

use super::sprite;
use sdl2::rect::Point;

const MAX_ENEMIES: usize = 16;

pub struct EnemySpawner;

impl<'a> System<'a> for EnemySpawner {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>, ReadStorage<'a, Enemy>);
    fn run(&mut self, (entities, lazy, enemies): Self::SystemData) {
        if enemies.join().count() >= MAX_ENEMIES {
            return;
        }

        let position = Point::new(
            thread_rng().gen_range(-200..200),
            thread_rng().gen_range(-200..200),
        );
        let enemy_animation = sprite::enemy_animation();
        // TODO: Inject enemies
        // TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        lazy.create_entity(&entities)
            .with(AIControlled)
            .with(Enemy)
            .with(Position(position))
            .with(Velocity {
                speed: 0,
                direction: Direction::Right,
            })
            .with(enemy_animation.right_frames[0].clone())
            .with(enemy_animation)
            .build();
    }
}
