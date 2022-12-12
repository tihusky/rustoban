mod game;
mod utils;

mod prelude {
    pub use crate::game::gfx::ui::*;
    pub use crate::game::gfx::*;
    pub use crate::game::level::*;
    pub use crate::game::player::*;
    pub use crate::game::*;
    pub use crate::state::*;
    pub use crate::utils::*;
    pub use ggez::conf::{WindowMode, WindowSetup};
    pub use ggez::event::{self, EventHandler, EventLoop};
    pub use ggez::graphics::*;
    pub use ggez::input::keyboard::KeyCode;
    pub use ggez::*;
    pub use uuid::Uuid;

    // In pixels
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;

    // In tiles
    pub const WINDOW_WIDTH: i32 = 40;
    pub const WINDOW_HEIGHT: i32 = 25;
}

use prelude::*;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Rustoban", "Mirko Förster")
        .window_setup(WindowSetup::default().title("Rustoban"))
        .window_mode(
            WindowMode::default()
                .dimensions(
                    (TILE_WIDTH * WINDOW_WIDTH) as f32,
                    (TILE_HEIGHT * WINDOW_HEIGHT) as f32,
                )
                .resizable(false),
        )
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context!");

    let state = State::new(&mut ctx).expect("Could not initialize game state!");

    event::run(ctx, event_loop, state);
}
