use crate::prelude::*;

mod level_manager;

pub use level_manager::*;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn in_bounds(&self, point: Point2D) -> bool {
        (point.x >= 0 && point.x < self.width) && (point.y >= 0 && point.y < self.height)
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        let idx = (self.width * y + x) as usize;

        self.tiles.get(idx)
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let mut mb = MeshBuilder::new();
        let x_offset = (WINDOW_WIDTH - self.width) / 2;
        let y_offset = (WINDOW_HEIGHT - self.height) / 2;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.get_tile(x, y) {
                    let color = match tile {
                        TileType::Wall => Color::from_rgb(25, 25, 25),
                        TileType::Floor => Color::from_rgb(85, 85, 85),
                        TileType::Target => Color::from_rgb(0, 165, 74),
                    };
                    let rect = Rect::new_i32(
                        TILE_WIDTH * (x + x_offset),
                        TILE_HEIGHT * (y + y_offset),
                        TILE_WIDTH,
                        TILE_HEIGHT,
                    );

                    mb.rectangle(DrawMode::fill(), rect, color)?;
                }
            }
        }

        let mesh = Mesh::from_data(ctx, mb.build());

        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }
}
