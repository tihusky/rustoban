use std::ops::SubAssign;

use crate::mint::Point2;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Point2D) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::Sub for Point2D {
    type Output = Self;

    fn sub(self, rhs: Point2D) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Into<Point2<f32>> for Point2D {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

pub fn point_to_pixels(point: Point2D) -> (i32, i32) {
    (TILE_WIDTH * point.x, TILE_HEIGHT * point.y)
}
