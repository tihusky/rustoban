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
    current_level: usize,
    player: Point2D,
    boxes: Vec<Point2D>,
}

impl State {
    pub fn new() -> Self {
        let mut lm = LevelManager::new();
        let current_level = 1;

        lm.load_from_file("./resources/levels.txt")
            .expect("Could not read levels file");

        let player = lm.get_level(current_level).unwrap().player;
        let boxes = lm.get_level(current_level).unwrap().boxes.clone();

        Self {
            levels: lm,
            current_level,
            player,
            boxes,
        }
    }

    fn draw_player(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        // Player rect should be half the size of a tile and centered on
        // the tile the player is standing on
        let (l_width, l_height) = self.get_current_level().unwrap().get_dimensions();
        let x_offset = (WINDOW_WIDTH - l_width) / 2 * TILE_WIDTH;
        let y_offset = (WINDOW_HEIGHT - l_height) / 2 * TILE_HEIGHT;
        let (tile_x, tile_y) = point_to_pixels(self.player);
        let rect_x = x_offset + tile_x + TILE_WIDTH / 4;
        let rect_y = y_offset + tile_y + TILE_HEIGHT / 4;

        let mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(rect_x, rect_y, TILE_WIDTH / 2, TILE_HEIGHT / 2),
            Color::WHITE,
        )?;

        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }

    fn draw_boxes(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        // Box rect should be half the size of a tile and centered on
        // the tile the box is standing on
        let (l_width, l_height) = self.get_current_level().unwrap().get_dimensions();
        let x_offset = (WINDOW_WIDTH - l_width) / 2 * TILE_WIDTH;
        let y_offset = (WINDOW_HEIGHT - l_height) / 2 * TILE_HEIGHT;

        for b in &self.boxes {
            let (tile_x, tile_y) = point_to_pixels(*b);
            let rect_x = x_offset + tile_x + TILE_WIDTH / 4;
            let rect_y = y_offset + tile_y + TILE_HEIGHT / 4;

            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new_i32(rect_x, rect_y, TILE_WIDTH / 2, TILE_HEIGHT / 2),
                Color::from_rgb(96, 47, 0),
            )?;

            canvas.draw(&mesh, DrawParam::default());
        }

        Ok(())
    }

    fn get_current_level(&self) -> Option<&Level> {
        self.levels.get_level(self.current_level)
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let level = self.levels.get_level(self.current_level).unwrap();
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        level.draw(ctx, &mut canvas)?;
        self.draw_player(ctx, &mut canvas)?;
        self.draw_boxes(ctx, &mut canvas)?;

        canvas.finish(ctx)?;

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
