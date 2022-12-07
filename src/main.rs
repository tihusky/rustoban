mod game;
mod utils;

mod prelude {
    pub use crate::game::level::*;
    pub use crate::game::*;
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GameState {
        Playing,
        Solved,
    }
}

use prelude::*;

struct State {
    levels: LevelManager,
    current_level: usize,
    player: GameEntity,
    boxes: Vec<GameEntity>,
    moves: Vec<PlayerMove>,
    game_state: GameState,
}

impl State {
    pub fn new() -> Self {
        let mut lm = LevelManager::new();
        let current_level = 0;

        lm.load_from_file("./resources/levels.txt")
            .expect("Could not read levels file");

        let player = GameEntity::new(lm.get_level(current_level).unwrap().player);
        let mut boxes = Vec::new();

        for box_pos in &lm.get_level(current_level).unwrap().boxes {
            boxes.push(GameEntity::new(*box_pos));
        }

        Self {
            levels: lm,
            current_level,
            player,
            boxes,
            moves: Vec::new(),
            game_state: GameState::Playing,
        }
    }

    fn get_current_level(&self) -> Option<&Level> {
        self.levels.get_level(self.current_level)
    }

    fn update_playing(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Back) {
            self.undo_last_move();
        }

        let delta = if ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            Point2D { x: -1, y: 0 }
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            Point2D { x: 1, y: 0 }
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            Point2D { x: 0, y: -1 }
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            Point2D { x: 0, y: 1 }
        } else {
            Point2D { x: 0, y: 0 }
        };

        if delta.x != 0 || delta.y != 0 {
            let level = self.get_current_level().unwrap();
            let player_dest = self.player.position + delta;

            if level.is_accessible(player_dest) {
                let mut boxes = self.boxes.clone();

                if let Some(b) = boxes.iter_mut().find(|b| b.position == player_dest) {
                    let box_dest = b.position + delta;
                    let contains_box = self
                        .boxes
                        .iter()
                        .map(|b| b.position)
                        .any(|pos| pos == box_dest);

                    if level.is_accessible(box_dest) && !contains_box {
                        b.position = box_dest;

                        self.moves.push(PlayerMove {
                            delta,
                            box_id: Some(b.id),
                        });

                        self.boxes = boxes;
                        self.player.position = player_dest;
                    }
                } else {
                    self.moves.push(PlayerMove {
                        delta,
                        box_id: None,
                    });

                    self.player.position = player_dest;
                }
            }
        }
    }

    fn update_solved(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
            self.current_level = (self.current_level + 1) % self.levels.num_levels();

            let level = self.get_current_level().unwrap();
            let mut boxes: Vec<GameEntity> = Vec::new();

            for pos in &level.boxes {
                boxes.push(GameEntity::new(*pos));
            }

            self.player = GameEntity::new(level.player);
            self.boxes = boxes;
            self.moves.clear();
            self.game_state = GameState::Playing;
        }
    }

    fn draw_playing(&mut self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        self.get_current_level().unwrap().draw(ctx, canvas)?;
        self.draw_player(ctx, canvas)?;
        self.draw_boxes(ctx, canvas)?;

        Ok(())
    }

    fn draw_player(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        // Player rect should be half the size of a tile and centered on
        // the tile the player is standing on
        let (l_width, l_height) = self.get_current_level().unwrap().get_dimensions();
        let x_offset = (WINDOW_WIDTH - l_width) / 2 * TILE_WIDTH;
        let y_offset = (WINDOW_HEIGHT - l_height) / 2 * TILE_HEIGHT;
        let (tile_x, tile_y) = point_to_pixels(self.player.position);
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
            let (tile_x, tile_y) = point_to_pixels(b.position);
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

    fn draw_solved(&self, canvas: &mut Canvas) -> GameResult {
        let mut text = Text::new("Well done, you solved this level!");
        let (center_x, center_y) = point_to_pixels(Point2D {
            x: WINDOW_WIDTH / 2,
            y: WINDOW_HEIGHT / 2,
        });

        text.set_layout(TextLayout::center());
        text.add("\nPress ENTER to play the next level");

        canvas.draw(
            &text,
            DrawParam::default().dest([center_x as f32, center_y as f32]),
        );

        Ok(())
    }

    fn undo_last_move(&mut self) {
        if let Some(last_move) = self.moves.pop() {
            self.player.position -= last_move.delta;

            if let Some(id) = last_move.box_id {
                let b = self.boxes.iter_mut().find(|b| b.id == id).unwrap();

                b.position -= last_move.delta;
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let level = self.get_current_level().unwrap();

        if self
            .boxes
            .iter()
            .all(|b| level.targets.contains(&b.position))
        {
            self.game_state = GameState::Solved;
        }

        match self.game_state {
            GameState::Playing => self.update_playing(ctx),
            GameState::Solved => self.update_solved(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.game_state {
            GameState::Playing => self.draw_playing(ctx, &mut canvas)?,
            GameState::Solved => self.draw_solved(&mut canvas)?,
        }

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
