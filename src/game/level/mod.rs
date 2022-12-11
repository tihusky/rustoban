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

    pub fn is_surrounded(&self, point: Point2D) -> bool {
        let mut wall_left = false;

        for x in 0..point.x {
            if let Some(tile) = self.get_tile(x, point.y) {
                if *tile == TileType::Wall {
                    wall_left = true;
                    break;
                }
            }
        }

        let mut wall_right = false;

        for x in point.x + 1..self.width {
            if let Some(tile) = self.get_tile(x, point.y) {
                if *tile == TileType::Wall {
                    wall_right = true;
                    break;
                }
            }
        }

        let mut wall_top = false;

        for y in 0..point.y {
            if let Some(tile) = self.get_tile(point.x, y) {
                if *tile == TileType::Wall {
                    wall_top = true;
                    break;
                }
            }
        }

        let mut wall_bottom = false;

        for y in point.y + 1..self.height {
            if let Some(tile) = self.get_tile(point.x, y) {
                if *tile == TileType::Wall {
                    wall_bottom = true;
                    break;
                }
            }
        }

        wall_left && wall_right && wall_top && wall_bottom
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
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
                    let scale_x = TILE_WIDTH as f32 / bg_image.width() as f32;
                    let scale_y = TILE_HEIGHT as f32 / bg_image.height() as f32;

                    canvas.draw(
                        bg_image,
                        DrawParam::default().dest(dest).scale([scale_x, scale_y]),
                    );

                    if *tile == TileType::Wall || *tile == TileType::Target {
                        let image = if *tile == TileType::Wall {
                            sprites.get_sprite("wall").unwrap()
                        } else {
                            sprites.get_sprite("target").unwrap()
                        };

                        let scale_x = TILE_WIDTH as f32 / bg_image.width() as f32;
                        let scale_y = TILE_HEIGHT as f32 / bg_image.height() as f32;

                        canvas.draw(
                            image,
                            DrawParam::default()
                                .dest(Point2D {
                                    x: TILE_WIDTH * (x + x_offset),
                                    y: TILE_HEIGHT * (y + y_offset),
                                })
                                .scale([scale_x, scale_y]),
                        );
                    }
                }
            }
        }
    }
}
