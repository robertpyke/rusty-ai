use crate::components::*;

use sdl2::rect::Rect;

const ENEMY_SPRITESHEET: usize = 1;
pub const TEXTURE_PATHS: [&str; 2] = ["assets/bardo.png", "assets/reaper.png"];

pub fn enemy_animation() -> MovementAnimation {
    let enemy_top_left_frame = Rect::new(0, 0, 32, 36);
    return MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            ENEMY_SPRITESHEET,
            enemy_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            ENEMY_SPRITESHEET,
            enemy_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            ENEMY_SPRITESHEET,
            enemy_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            ENEMY_SPRITESHEET,
            enemy_top_left_frame,
            Direction::Right,
        ),
    };
}

/// Returns the row of the spritesheet corresponding to the given direction
pub fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// Create animation frames for the standard character spritesheet
pub fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    // All assumptions about the spritesheets are now encapsulated in this function instead of in
    // the design of our entire system. We can always replace this function, but replacing the
    // entire system is harder.

    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }
    frames
}
