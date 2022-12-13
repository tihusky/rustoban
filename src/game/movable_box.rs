use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovableBox {
    id: Uuid,
    position: Point2D,
}

impl MovableBox {
    pub fn new(pos: Point2D) -> Self {
        Self {
            id: Uuid::new_v4(),
            position: pos,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_position(&self) -> &Point2D {
        &self.position
    }

    pub fn get_position_mut(&mut self) -> &mut Point2D {
        &mut self.position
    }

    pub fn can_be_moved(&self, delta: Point2D, level: &Level, boxes: &[MovableBox]) -> bool {
        let destination = self.position + delta;
        let is_occupied = boxes.iter().any(|b| b.position == destination);

        level.is_accessible(destination) && !is_occupied
    }

    pub fn is_on_target(&self, targets: &[Point2D]) -> bool {
        targets.contains(&self.position)
    }

    pub fn draw(
        &self,
        sprites: &SpriteManager,
        canvas: &mut Canvas,
        offset: Point2D,
        targets: &[Point2D],
    ) {
        let sprite = if self.is_on_target(targets) {
            sprites.get_sprite("box_on_target").unwrap()
        } else {
            sprites.get_sprite("box").unwrap()
        };

        let scale = get_scaling_factors(sprite);
        let dest = Point2D {
            x: TILE_WIDTH * (self.position.x + offset.x),
            y: TILE_HEIGHT * (self.position.y + offset.y),
        };

        canvas.draw(sprite, DrawParam::default().scale(scale).dest(dest));
    }
}
