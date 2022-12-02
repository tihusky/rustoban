mod game;
mod utils;

mod prelude {
    pub use crate::game::level::*;
    pub use crate::utils::*;
    pub use ggez::conf::{WindowMode, WindowSetup};
    pub use ggez::event::{self, EventHandler, EventLoop};
    pub use ggez::graphics::*;
    pub use ggez::*;

    // In pixels
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;

    // In tiles
    pub const WINDOW_WIDTH: i32 = 40;
    pub const WINDOW_HEIGHT: i32 = 25;
}

use prelude::*;

struct State {
    levels: LevelManager,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            levels: LevelManager::new(),
        };

        s.levels.load_from_file("./resources/levels.txt");

        s
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let (w, h) = point_to_pixels(Point2D {
        x: WINDOW_WIDTH,
        y: WINDOW_HEIGHT,
    });

    let (ctx, event_loop) = ContextBuilder::new("Rustoban", "Mirko Förster")
        .window_setup(WindowSetup::default().title("Rustoban"))
        .window_mode(
            WindowMode::default()
                .dimensions(w as f32, h as f32)
                .resizable(false),
        )
        .build()
        .expect("Could not create ggez context!");

    event::run(ctx, event_loop, State::new());
}
