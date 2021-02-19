use specs::prelude::*;

use crate::components::*;
use rand::prelude::*;

use super::sprite;
use sdl2::rect::Point;

const MAX_ENEMIES: usize = 50;

pub struct EnemySpawner;

impl<'a> System<'a> for EnemySpawner {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Telemetry>,
    );
    fn run(&mut self, (entities, lazy, enemies, mut telemetries): Self::SystemData) {
        let enemy_count = enemies.join().count();
        if enemy_count >= MAX_ENEMIES {
            return;
        }

        let position = Point::new(
            thread_rng().gen_range(-200..200),
            thread_rng().gen_range(-200..200),
        );
        let enemy_animation = sprite::enemy_animation();
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
        match (&mut telemetries).join().last() {
            Some(telemetry) => telemetry.enemy_spawned += 1,
            None => eprintln!("Telemetry Missing"),
        }
    }
}
