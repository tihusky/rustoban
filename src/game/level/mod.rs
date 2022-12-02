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
}
