use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Solved,
}

pub struct State {
    levels: LevelManager,
    sprites: SpriteManager,
    current_level: usize,
    player: Player,
    boxes: Vec<MovableBox>,
    moves: Vec<PlayerMove>,
    game_state: GameState,
}

impl State {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        let mut lm = LevelManager::new();
        let current_level = 0;

        lm.load_from_file(ctx, "/levels.txt")?;

        let mut sm = SpriteManager::new();

        sm.add_sprite(ctx, "wall", "/graphics/wall.png")?;
        sm.add_sprite(ctx, "floor", "/graphics/floor.png")?;
        sm.add_sprite(ctx, "target", "/graphics/target.png")?;
        sm.add_sprite(ctx, "player_up", "/graphics/player_up.png")?;
        sm.add_sprite(ctx, "player_right", "/graphics/player_right.png")?;
        sm.add_sprite(ctx, "player_down", "/graphics/player_down.png")?;
        sm.add_sprite(ctx, "player_left", "/graphics/player_left.png")?;
        sm.add_sprite(ctx, "box", "/graphics/box01.png")?;
        sm.add_sprite(ctx, "box_on_target", "/graphics/box02.png")?;

        match graphics::FontData::from_path(ctx, "/fonts/videotype.ttf") {
            Ok(font) => ctx.gfx.add_font("Videotype", font),
            Err(e) => return Err(e.to_string()),
        }

        let player_pos = lm.get_level(current_level).unwrap().player;
        let mut boxes = Vec::new();

        for box_pos in &lm.get_level(current_level).unwrap().boxes {
            boxes.push(MovableBox::new(*box_pos));
        }

        Ok(Self {
            levels: lm,
            sprites: sm,
            current_level,
            player: Player::new(player_pos),
            boxes,
            moves: Vec::new(),
            game_state: GameState::Playing,
        })
    }

    fn get_current_level(&self) -> Option<&Level> {
        self.levels.get_level(self.current_level)
    }

    fn update_playing(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Back) {
            self.undo_last_move();
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::R) {
            self.reset_level();
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
            let level = self.get_current_level().unwrap().clone();

            if let Ok(player_move) = self.player.try_move(delta, &level, &self.boxes) {
                if let Some(box_id) = player_move.box_id {
                    self.boxes
                        .iter_mut()
                        .filter(|b| *b.get_id() == box_id)
                        .for_each(|b| *b.get_position_mut() += delta);
                }

                self.moves.push(player_move);
            }
        }
    }

    fn update_solved(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Return)
            || ctx.keyboard.is_key_just_pressed(KeyCode::NumpadEnter)
        {
            self.current_level = (self.current_level + 1) % self.levels.num_levels();
            self.reset_level();
            self.game_state = GameState::Playing;
        }
    }

    fn draw_playing(&mut self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let level = self.get_current_level().unwrap();

        // Calculate offset to draw player and boxes relative to the level
        let offset = Point2D {
            x: (WINDOW_WIDTH - level.width) / 2,
            y: (WINDOW_HEIGHT - level.height) / 2,
        };

        level.draw(&self.sprites, canvas, offset);
        self.player.draw(&self.sprites, canvas, offset);

        for b in &self.boxes {
            b.draw(&self.sprites, canvas, offset, &level.targets);
        }

        let mut hint_texts: Vec<TextBlock> = Vec::new();

        hint_texts.push(TextBlock::new(
            TextFragment::new("Press BACKSPACE to undo last move")
                .font("Videotype")
                .scale(20.0),
            (0.0, 0.0, 0.0, 0.0),
            TextAlign::Begin,
        ));
        hint_texts.push(TextBlock::new(
            TextFragment::new("Press R to reset level")
                .font("Videotype")
                .scale(20.0),
            (10.0, 0.0, 0.0, 0.0),
            TextAlign::Begin,
        ));

        let mut move_texts: Vec<TextBlock> = Vec::new();

        move_texts.push(TextBlock::new(
            TextFragment::new(&format!("Moves: {}", self.moves.len()))
                .font("Videotype")
                .scale(20.0),
            (0.0, 0.0, 0.0, 0.0),
            TextAlign::End,
        ));

        print_spaced(ctx, canvas, &hint_texts, Point2D { x: 32, y: 24 });
        print_spaced(
            ctx,
            canvas,
            &move_texts,
            Point2D {
                x: (WINDOW_WIDTH * TILE_WIDTH - 32),
                y: 24,
            },
        );

        Ok(())
    }

    fn draw_solved(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let origin = Point2D {
            x: WINDOW_WIDTH / 2 * TILE_WIDTH,
            y: 0,
        };
        let mut blocks: Vec<TextBlock> = Vec::new();

        blocks.push(TextBlock::new(
            TextFragment::new("Well done, you solved this level!")
                .font("Videotype")
                .color(Color::GREEN)
                .scale(24.0),
            (48.0, 0.0, 0.0, 0.0),
            TextAlign::Middle,
        ));
        blocks.push(TextBlock::new(
            TextFragment::new(&format!("Number of Moves: {}", self.moves.len()))
                .font("Videotype")
                .scale(48.0),
            (96.0, 0.0, 0.0, 0.0),
            TextAlign::Middle,
        ));
        blocks.push(TextBlock::new(
            TextFragment::new("Press ENTER to play the next level").font("Videotype"),
            (96.0, 0.0, 0.0, 0.0),
            TextAlign::Middle,
        ));

        print_spaced(ctx, canvas, &blocks, origin);

        Ok(())
    }

    fn undo_last_move(&mut self) {
        if let Some(last_move) = self.moves.pop() {
            self.player.position -= last_move.delta;

            if let Some(id) = last_move.box_id {
                let b = self.boxes.iter_mut().find(|b| *b.get_id() == id).unwrap();

                *b.get_position_mut() -= last_move.delta;
            }
        }
    }

    fn reset_level(&mut self) {
        let level = self.get_current_level().unwrap();
        let mut boxes: Vec<MovableBox> = Vec::new();

        for pos in &level.boxes {
            boxes.push(MovableBox::new(*pos));
        }

        self.player = Player::new(level.player);
        self.boxes = boxes;
        self.moves.clear();
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let level = self.get_current_level().unwrap();

        if level.is_solved(&self.boxes) {
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
            GameState::Solved => self.draw_solved(ctx, &mut canvas)?,
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}
