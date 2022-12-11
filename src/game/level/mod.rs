use crate::prelude::*;

mod level_manager;

pub use level_manager::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Target,
}

#[derive(Clone, Debug)]
pub struct Level {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub player: Point2D,
    pub boxes: Vec<Point2D>,
    pub targets: Vec<Point2D>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            player: Point2D { x: 0, y: 0 },
            boxes: Vec::new(),
            targets: Vec::new(),
        }
    }

    pub fn is_accessible(&self, point: Point2D) -> bool {
        if let Some(tile) = self.get_tile(point.x, point.y) {
            return *tile != TileType::Wall;
        }

        false
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        let idx = (self.width * y + x) as usize;

        self.tiles.get(idx)
    }

    pub fn draw(&self, sprites: &SpriteManager, canvas: &mut Canvas) {
        // Calculate where to start drawing so the level is centered in the window
        let x_offset = (WINDOW_WIDTH - self.width) / 2;
        let y_offset = (WINDOW_HEIGHT - self.height) / 2;

        let bg_image = sprites.get_sprite("floor").unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.get_tile(x, y) {
                    let dest = Point2D {
                        x: TILE_WIDTH * (x + x_offset),
                        y: TILE_HEIGHT * (y + y_offset),
                    };

                    canvas.draw(
                        bg_image,
                        DrawParam::default()
                            .dest(dest)
                            .scale(get_scaling_factors(bg_image)),
                    );

                    if *tile == TileType::Wall || *tile == TileType::Target {
                        let image = if *tile == TileType::Wall {
                            sprites.get_sprite("wall").unwrap()
                        } else {
                            sprites.get_sprite("target").unwrap()
                        };

                        canvas.draw(
                            image,
                            DrawParam::default()
                                .dest(Point2D {
                                    x: TILE_WIDTH * (x + x_offset),
                                    y: TILE_HEIGHT * (y + y_offset),
                                })
                                .scale(get_scaling_factors(image)),
                        );
                    }
                }
            }
        }
    }
}
