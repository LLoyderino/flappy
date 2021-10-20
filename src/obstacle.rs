use bracket_lib::prelude::*;

use crate::game::*;
use crate::player::*;

pub struct Obstacle {
    pub x: i32,
    pub gap_y: i32,
    pub size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut rand = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: rand.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        ctx.set_active_console(0);

        for y in 0..self.gap_y - half_size {
            ctx.add_sprite(
                Rect::with_size(screen_x, y, 2, 2),
                400 - y,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                Sprites::Block as usize,
            )
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.add_sprite(
                Rect::with_size(screen_x, y, 2, 2),
                400 - y,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                Sprites::Block as usize,
            )
        }

        ctx.set_active_console(1);
    }

    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
