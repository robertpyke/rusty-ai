use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use specs::prelude::*;

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Telemetry>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    (positions, sprites, telemetries): SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&positions, &sprites).join() {
        let current_frame = sprite.region;

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    // Render Telemetry Info
    match (&telemetries).join().last() {
        Some(telemetry) => {
            let text = format!(
                "Spawned: {}\nOOB: {}\nCollided: {}",
                telemetry.enemy_spawned, telemetry.enemy_oob, telemetry.enemy_collisions
            );
            let surface = font
                .render(&text)
                .blended_wrapped(Color::RGBA(0, 0, 0, 255), super::WORLD_WIDTH)
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            let TextureQuery { width, height, .. } = texture.query();

            let padding = 10;
            let target = super::rect!(padding, padding, width + padding, height + padding);
            canvas.copy(&texture, None, Some(target))?;
        }
        None => eprintln!("Telemetry Missing"),
    }

    canvas.present();

    Ok(())
}
