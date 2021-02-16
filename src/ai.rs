use specs::prelude::*;
use rand::prelude::*;

use crate::components::*;

const ENEMY_MOVEMENT_SPEED: i32 = 8;
const HERO_MOVEMENT_SPEED: i32 = 3;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        ReadStorage<'a, AIControlled>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Hero>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let mut rng = thread_rng();
        for (_, _, vel) in (&data.0, &data.1, &mut data.4).join() {
            if rng.gen_range(0..2) == 0 {
                vel.speed = ENEMY_MOVEMENT_SPEED;
                vel.direction = match rng.gen_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => unreachable!(),
                }
            }
        }

        for (_, _, hero_pos, hero_vel) in (&data.0, &data.2, &data.3, &mut data.4).join() {
            let hero_x = hero_pos.0.x;
            let hero_y = hero_pos.0.y;

            // Find the nearest enemy
            let mut nearest_enemy_pos: Option<Position> = None;
            let mut nearest_distance = f32::MAX;
            for (_, enemy_pos) in (&data.1, &data.3).join() {
                let enemy_x = enemy_pos.0.x;
                let enemy_y = enemy_pos.0.y;

                let x_delta = (enemy_x - hero_x).abs();
                let y_delta = (enemy_y - hero_y).abs();
                let c_squared = (x_delta.pow(2) + y_delta.pow(2)) as f32;
                let distance = c_squared.sqrt();

                match nearest_enemy_pos {
                    None | _ if distance < nearest_distance => {
                        nearest_enemy_pos = Some(enemy_pos.clone());
                        nearest_distance = distance;
                    },
                    _ => {
                        // no-op (not nearer)
                    }
                }
            }

            match nearest_enemy_pos {
                Some(nearest_enemy_pos) => {
                    println!("Nearest enemy: {:?}", nearest_enemy_pos);
                    let enemy_x = nearest_enemy_pos.0.x;
                    let enemy_y = nearest_enemy_pos.0.y;
                    let x_delta = enemy_x - hero_x;
                    let x_delta_abs = x_delta.abs();
                    let y_delta = enemy_y - hero_y;
                    let y_delta_abs = y_delta.abs();
                    match x_delta_abs >= y_delta_abs {
                        true if x_delta_abs > 0 => {
                            // head in the X
                            hero_vel.speed = match x_delta_abs {
                                0..=HERO_MOVEMENT_SPEED => x_delta_abs,
                                _ => HERO_MOVEMENT_SPEED
                            };
                            match x_delta {
                                i32::MIN..=-1 => {
                                    hero_vel.direction = Direction::Left;
                                }
                                0..=i32::MAX => {
                                    hero_vel.direction = Direction::Right;
                                },
                            }
                        }, 
                        false if y_delta_abs > 0 => {
                            // head in the Y
                            hero_vel.speed = match y_delta_abs {
                                0..=HERO_MOVEMENT_SPEED => y_delta_abs,
                                _ => HERO_MOVEMENT_SPEED
                            };
                            match y_delta {
                                i32::MIN..=-1 => {
                                    hero_vel.direction = Direction::Up;
                                }
                                0..=i32::MAX => {
                                    hero_vel.direction = Direction::Down;
                                },
                            }
                        },
                        _ => {
                            println!("Not moving, at enemy.");
                            // Stop Moving
                            hero_vel.speed = 0;
                        }
                    }
                }
                
                None => {
                    println!("Not moving, no enemy.");
                    // Stop Moving
                    hero_vel.speed = 0;
                }
            }
        }
    }
}