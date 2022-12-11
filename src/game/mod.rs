use crate::prelude::*;

pub mod gfx;
pub mod level;
pub mod state;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameEntity {
    pub id: Uuid,
    pub position: Point2D,
}

impl GameEntity {
    pub fn new(position: Point2D) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerMove {
    pub delta: Point2D,
    pub box_id: Option<Uuid>,
}
