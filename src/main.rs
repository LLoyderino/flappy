#![windows_subsystem = "windows"]
#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

mod game;
mod obstacle;
mod player;

use game::*;

bracket_terminal::embedded_resource!(BLOCK, "../asset/sprite.png");

fn main() -> BError {
    bracket_terminal::link_resource!(BLOCK, "asset/sprite.png");

    let context = BTermBuilder::new()
        .with_sprite_console(80, 50, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_title("Flappy Dragon")
        .with_sprite_sheet(
            SpriteSheet::new("asset/sprite.png")
                .add_sprite(Rect::with_size(0, 0, 16, 16))
                .add_sprite(Rect::with_size(16, 0, 16, 16)),
        )
        .with_vsync(false)
        .build()?;

    main_loop(context, State::new())
}
