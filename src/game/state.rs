use crate::prelude::*;

pub struct State {
    levels: LevelManager,
    sprites: SpriteManager,
    current_level: usize,
    player: GameEntity,
    boxes: Vec<GameEntity>,
    moves: Vec<PlayerMove>,
    game_state: GameState,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let mut lm = LevelManager::new();
        let current_level = 0;

        lm.load_from_file("./resources/levels.txt")
            .expect("Could not read levels file");

        let mut sm = SpriteManager::new();

        sm.add_sprite(ctx, "wall", "/wall.png")
            .expect("Error loading wall sprite");
        sm.add_sprite(ctx, "floor", "/floor.png")
            .expect("Error loading floor sprite");
        sm.add_sprite(ctx, "target", "/target.png")
            .expect("Error loading target sprite");
        sm.add_sprite(ctx, "player", "/player.png")
            .expect("Error loading player sprite");
        sm.add_sprite(ctx, "box", "/box01.png")
            .expect("Error loading box sprite");
        sm.add_sprite(ctx, "box_on_target", "/box02.png")
            .expect("Error loading box on target sprite");

        ctx.gfx.add_font(
            "Videotype",
            graphics::FontData::from_path(ctx, "/videotype.ttf").expect("Could not load font"),
        );

        let player = GameEntity::new(lm.get_level(current_level).unwrap().player);
        let mut boxes = Vec::new();

        for box_pos in &lm.get_level(current_level).unwrap().boxes {
            boxes.push(GameEntity::new(*box_pos));
        }

        Self {
            levels: lm,
            sprites: sm,
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
            self.reset_level();
            self.game_state = GameState::Playing;
        }
    }

    fn draw_playing(&mut self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        self.get_current_level()
            .unwrap()
            .draw(&self.sprites, canvas);
        self.draw_player(&self.sprites, canvas);
        self.draw_boxes(&self.sprites, canvas);

        let mut hint_blocks: Vec<TextBlock> = Vec::new();

        hint_blocks.push(TextBlock::new(
            TextFragment::new("Press BACKSPACE to undo last move")
                .font("Videotype")
                .scale(20.0),
            (0.0, 0.0, 0.0, 0.0),
            TextAlign::Begin,
        ));
        hint_blocks.push(TextBlock::new(
            TextFragment::new("Press R to reset level")
                .font("Videotype")
                .scale(20.0),
            (10.0, 0.0, 0.0, 0.0),
            TextAlign::Begin,
        ));

        let mut move_blocks: Vec<TextBlock> = Vec::new();

        move_blocks.push(TextBlock::new(
            TextFragment::new(&format!("Moves: {}", self.moves.len()))
                .font("Videotype")
                .scale(20.0),
            (0.0, 0.0, 0.0, 0.0),
            TextAlign::End,
        ));

        print_spaced(ctx, canvas, &hint_blocks, Point2D { x: 32, y: 24 });
        print_spaced(
            ctx,
            canvas,
            &move_blocks,
            Point2D {
                x: (WINDOW_WIDTH * TILE_WIDTH - 32),
                y: 24,
            },
        );

        Ok(())
    }

    fn draw_player(&self, sprites: &SpriteManager, canvas: &mut Canvas) {
        let level = self.get_current_level().unwrap();
        let x_offset = (WINDOW_WIDTH - level.width) / 2;
        let y_offset = (WINDOW_HEIGHT - level.height) / 2;
        let image = sprites.get_sprite("player").unwrap();

        canvas.draw(
            image,
            DrawParam::default()
                .dest(Point2D {
                    x: TILE_WIDTH * (self.player.position.x + x_offset),
                    y: TILE_HEIGHT * (self.player.position.y + y_offset),
                })
                .scale(get_scaling_factors(image)),
        );
    }

    fn draw_boxes(&self, sprites: &SpriteManager, canvas: &mut Canvas) {
        let level = self.get_current_level().unwrap();
        let x_offset = (WINDOW_WIDTH - level.width) / 2;
        let y_offset = (WINDOW_HEIGHT - level.height) / 2;

        for b in &self.boxes {
            let image = if level.targets.contains(&b.position) {
                sprites.get_sprite("box_on_target").unwrap()
            } else {
                sprites.get_sprite("box").unwrap()
            };

            canvas.draw(
                image,
                DrawParam::default()
                    .dest(Point2D {
                        x: TILE_WIDTH * (b.position.x + x_offset),
                        y: TILE_HEIGHT * (b.position.y + y_offset),
                    })
                    .scale(get_scaling_factors(image)),
            )
        }
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
                let b = self.boxes.iter_mut().find(|b| b.id == id).unwrap();

                b.position -= last_move.delta;
            }
        }
    }

    fn reset_level(&mut self) {
        let level = self.get_current_level().unwrap();
        let mut boxes: Vec<GameEntity> = Vec::new();

        for pos in &level.boxes {
            boxes.push(GameEntity::new(*pos));
        }

        self.player = GameEntity::new(level.player);
        self.boxes = boxes;
        self.moves.clear();
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
            GameState::Solved => self.draw_solved(ctx, &mut canvas)?,
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}
