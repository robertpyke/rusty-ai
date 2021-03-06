mod ai;
mod animator;
mod components;
mod enemy_collider_purger;
mod enemy_oob_purger;
mod enemy_spawner;
mod keyboard;
mod physics;
mod renderer;
mod sprite;

use rand::prelude::*;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, InitFlag, LoadTexture};

use specs::prelude::*;

use crate::components::*;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

pub const WORLD_WIDTH: u32 = 800;
pub const WORLD_HEIGHT: u32 = 600;

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn initialize_player(world: &mut World, player_spritesheet: usize) {
    let player_top_left_frame =
        Rect::new(0, 0, sprite::HERO_FRAME_WIDTH, sprite::HERO_FRAME_HEIGHT);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: sprite::character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: sprite::character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: sprite::character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: sprite::character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };

    world
        .create_entity()
        .with(AIControlled)
        .with(Hero)
        .with(Position(Point::new(
            thread_rng().gen_range(-200..200),
            thread_rng().gen_range(-200..200),
        )))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(Telemetry {
            enemy_collisions: 0,
            enemy_oob: 0,
            enemy_spawned: 0,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("rusty-ai", WORLD_WIDTH, WORLD_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("assets/fonts/Roboto/Roboto-Regular.ttf", 20)?;
    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(enemy_spawner::EnemySpawner, "EnemySpawner", &[])
        .with(ai::AI, "AI", &["EnemySpawner"])
        .with(
            enemy_oob_purger::EnemyOOBPurger,
            "EnemyOOBPurger",
            &["EnemySpawner", "AI"],
        )
        .with(
            enemy_collider_purger::EnemyColliderPurger,
            "EnemyColliderPurger",
            &["EnemySpawner", "AI"],
        )
        .with(physics::Physics, "Physics", &["Keyboard", "AI"])
        .with(animator::Animator, "Animator", &["Keyboard", "AI"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    // Initialize resource
    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    let mut textures = Vec::with_capacity(sprite::TEXTURE_PATHS.len());
    for path in &sprite::TEXTURE_PATHS {
        textures.push(texture_creator.load_texture(path)?)
    }

    // First texture in textures array
    let player_spritesheet = 0;

    initialize_player(&mut world, player_spritesheet);

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let start_time = Instant::now();

        // None - no change, Some(MovementCommand) - perform movement
        let mut movement_command = None;
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        renderer::render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            &texture_creator,
            &font,
            world.system_data(),
        )?;

        let end_time = Instant::now();
        let difference = end_time.duration_since(start_time);
        println!("frame_time: {:?}", difference);
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
