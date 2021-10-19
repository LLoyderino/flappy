#![windows_subsystem = "windows"]

use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

enum Sprites {
    Block = 0,
    Grass = 1,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 30.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rand = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: rand.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
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

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
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

    fn play(&mut self, ctx: &mut BTerm) {
        clear_screen(ctx);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(1, 1, "Press SPACE to flap");
        ctx.print(1, 2, &format!("Score {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
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

    fn restart(&mut self, ctx: &mut BTerm) {
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

fn clear_screen(ctx: &mut BTerm) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();
}

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
