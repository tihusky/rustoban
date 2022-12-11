pub mod ui;

use crate::prelude::*;
use std::collections::HashMap;

pub struct SpriteManager {
    sprites: HashMap<String, Image>,
}

impl SpriteManager {
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    pub fn add_sprite(&mut self, ctx: &Context, key: &str, filename: &str) -> Result<(), String> {
        let image_res = Image::from_path(ctx, filename);

        if let Err(msg) = image_res {
            return Err(msg.to_string());
        }

        if self.sprites.contains_key(key) {
            return Err("Sprite with that name already exists".to_owned());
        }

        self.sprites.insert(String::from(key), image_res.unwrap());

        Ok(())
    }

    pub fn get_sprite(&self, name: &str) -> Option<&Image> {
        self.sprites.get(name)
    }
}
