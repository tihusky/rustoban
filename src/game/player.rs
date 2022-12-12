use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerMove {
    pub delta: Point2D,
    pub box_id: Option<Uuid>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub position: Point2D,
    direction: Direction,
}

impl Player {
    pub fn new(pos: Point2D) -> Self {
        Self {
            position: pos,
            direction: Direction::Down,
        }
    }

    pub fn try_move(
        &mut self,
        delta: Point2D,
        level: &Level,
        boxes: &Vec<GameEntity>,
    ) -> Result<PlayerMove, ()> {
        let player_dest = self.position + delta;

        if delta.x < 0 {
            self.direction = Direction::Left;
        } else if delta.x > 0 {
            self.direction = Direction::Right;
        } else if delta.y < 0 {
            self.direction = Direction::Up;
        } else if delta.y > 0 {
            self.direction = Direction::Down;
        }

        if !level.is_accessible(player_dest) {
            return Err(());
        }

        for b in boxes {
            if b.position == player_dest {
                let box_dest = b.position + delta;

                if !level.is_accessible(box_dest) {
                    return Err(());
                }

                self.position = player_dest;

                return Ok(PlayerMove {
                    delta,
                    box_id: Some(b.id),
                });
            }
        }

        self.position = player_dest;

        Ok(PlayerMove {
            delta,
            box_id: None,
        })
    }

    pub fn draw(&self, sprites: &SpriteManager, canvas: &mut Canvas, offset: Point2D) {
        let sprite_name = match self.direction {
            Direction::Up => "player_up",
            Direction::Right => "player_right",
            Direction::Down => "player_down",
            Direction::Left => "player_left",
        };
        let sprite = sprites
            .get_sprite(sprite_name)
            .expect(&format!("Error getting sprite: {}", sprite_name));

        let scale = get_scaling_factors(sprite);
        let dest = Point2D {
            x: TILE_WIDTH * (self.position.x + offset.x),
            y: TILE_HEIGHT * (self.position.y + offset.y),
        };

        canvas.draw(sprite, DrawParam::default().scale(scale).dest(dest));
    }
}
