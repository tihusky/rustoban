use crate::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct LevelManager {
    levels: Vec<Level>,
}

impl LevelManager {
    pub fn new() -> Self {
        Self { levels: Vec::new() }
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Error reading levels file");

            let levels: Vec<&str> = contents.split(';').collect();

            for level in levels {
                self.parse_level_string(level);
            }

            Ok(())
        } else {
            Err("Could not open levels file!".to_owned())
        }
    }

    fn parse_level_string(&mut self, level_str: &str) {
        let mut level = Level::new();

        // Because we know the level must be surrounded by walls every valid
        // row must contain at least two wall tiles. We can filter out lines
        // that don't contain any walls.
        let lines: Vec<&str> = level_str
            .split(char::is_control)
            .filter(|l| l.contains('#'))
            .collect();

        // A valid level consists of three or more rows.
        if lines.len() < 3 {
            return;
        }

        // Get the length of the longest row. This will be needed to determine
        // how many spaces to insert at the end of every row to ensure they all
        // have the same length.
        let max_len = lines.iter().map(|l| l.len()).max().unwrap();

        // Set the level dimensions.
        level.width = max_len as i32;
        level.height = lines.len() as i32;

        for (row, l) in lines.iter().enumerate() {
            let diff_len = max_len - l.len();

            for (col, c) in l.char_indices() {
                match c {
                    '#' => level.tiles.push(TileType::Wall),
                    ' ' => level.tiles.push(TileType::Floor),
                    '@' => {
                        level.player = Point2D {
                            x: col as i32,
                            y: row as i32,
                        };
                        level.tiles.push(TileType::Floor)
                    }
                    '+' => {
                        let p = Point2D {
                            x: col as i32,
                            y: row as i32,
                        };
                        level.player = p;
                        level.targets.push(p);
                        level.tiles.push(TileType::Target)
                    }
                    '$' => {
                        level.boxes.push(Point2D {
                            x: col as i32,
                            y: row as i32,
                        });
                        level.tiles.push(TileType::Floor)
                    }
                    '*' => {
                        let p = Point2D {
                            x: col as i32,
                            y: row as i32,
                        };
                        level.boxes.push(p);
                        level.targets.push(p);
                        level.tiles.push(TileType::Target)
                    }
                    '.' => {
                        level.targets.push(Point2D {
                            x: col as i32,
                            y: row as i32,
                        });
                        level.tiles.push(TileType::Target)
                    }
                    _ => (),
                }
            }

            for _ in 0..diff_len {
                level.tiles.push(TileType::Floor);
            }
        }

        self.levels.push(level);
    }

    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    pub fn get_level(&self, idx: usize) -> Option<&Level> {
        self.levels.get(idx)
    }
}
