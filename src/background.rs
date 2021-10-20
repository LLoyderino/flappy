use bracket_lib::prelude::*;

use crate::game::{Sprites, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn render(ctx: &mut BTerm) {
    ctx.set_active_console(0);

    // render sky
    for i in 0..SCREEN_WIDTH {
        for j in 0..SCREEN_HEIGHT - 1 {
            ctx.add_sprite(
                Rect::with_size(i, j, 1, 1),
                0,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                Sprites::Sky as usize,
            );
        }
    }

    // render grass top
    for i in 0..SCREEN_WIDTH - 1 {
        ctx.add_sprite(
            Rect::with_size(i, SCREEN_HEIGHT - 1, 1, 1),
            0,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            Sprites::GrassTop as usize,
        );
    }

    // render grass bot
    for i in 0..SCREEN_WIDTH {
        ctx.add_sprite(
            Rect::with_size(i, SCREEN_HEIGHT, 1, 1),
            0,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            Sprites::Grass as usize,
        );
    }

    ctx.set_active_console(1);
}
