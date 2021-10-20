use bracket_lib::prelude::*;

use crate::obstacle::Obstacle;
use crate::player::Player;
use crate::background;

#[allow(clippy::module_name_repetitions)]
pub enum GameMode {
    Menu,
    Playing,
    End,
}

pub enum Sprites {
    Block = 0,
    Grass = 1,
    GrassTop = 2,
    Player = 3,
    Sky = 4
}

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const FRAME_DURATION: f32 = 30.0;

pub struct State {
    pub player: Player,
    pub frame_time: f32,
    pub obstacle: Obstacle,
    pub mode: GameMode,
    pub score: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        clear_screen(ctx);
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        clear_screen(ctx);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity();
            self.obstacle.approach_player();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        ctx.print(1, 1, "Press SPACE to flap");
        ctx.print(1, 2, &format!("Score {}", self.score));

        // Background rendering
        background::render(ctx);

        // Obstacle rendering
        self.obstacle.render(ctx);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);
        }

        // Player rendering
        self.player.render(ctx);
        
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("Highscore: {}", self.score));
        ctx.print_centered(8, "(P) Play again");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn restart(&mut self, ctx: &mut BTerm) {
        clear_screen(ctx);
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

pub fn clear_screen(ctx: &mut BTerm) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();
}
