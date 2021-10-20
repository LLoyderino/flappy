use bracket_lib::prelude::*;

use crate::game::Sprites;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);

        ctx.add_sprite(
            Rect::with_size(self.x, self.y, 2, 1),
            2,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            Sprites::Player as usize,
        );

        ctx.set_active_console(1);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
