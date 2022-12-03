use crate::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum ReadMode {
    Command,
    Layout,
}

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

            let levels: Vec<&str> = contents.split_inclusive(":LEVELEND").collect();

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
        let mut mode = ReadMode::Command;
        let mut row = 0;

        let lines: Vec<&str> = level_str
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        for line in lines {
            // Needed because the read_layout method does not reset the mode
            // after reading the last layout line
            if line.starts_with(':') {
                mode = ReadMode::Command;
            }

            match mode {
                ReadMode::Command => self.execute_command(line, &mut level, &mut mode),
                ReadMode::Layout => self.read_layout(line, &mut row, &mut level),
            }
        }
    }

    fn execute_command(&mut self, line: &str, level: &mut Level, mode: &mut ReadMode) {
        let command = line.strip_prefix(':').unwrap();

        if command == "LEVELSTART" {
            *level = Level::new();
        } else if command == "LEVELEND" {
            if level.tiles.len() != (level.width * level.height) as usize {
                panic!("Level dimensions do not match number of tiles");
            } else if level.player == (Point2D { x: 0, y: 0 }) {
                panic!("Player spawn position not set");
            } else if level.boxes.len() < level.targets.len() {
                panic!("Level doesn't contain enough boxes");
            } else if level.targets.is_empty() {
                panic!("Level doesn't contain any targets");
            }

            self.levels.push(level.clone());
        } else if command == "LAYOUT" {
            *mode = ReadMode::Layout;
        } else if command.starts_with("WIDTH") || command.starts_with("HEIGHT") {
            let command_vec: Vec<&str> = command.split(' ').collect();

            if command_vec.len() != 2 {
                panic!("Invalid number of arguments to WIDTH / HEIGHT command");
            }

            let arg: i32 = if let Ok(val) = command_vec[1].parse() {
                val
            } else {
                panic!("Invalid type of argument to WIDTH / HEIGHT command");
            };

            if command_vec[0] == "WIDTH" {
                level.width = arg;
            } else if command_vec[0] == "HEIGHT" {
                level.height = arg;
            }
        }
    }

    fn read_layout(&self, line: &str, row: &mut i32, level: &mut Level) {
        if level.width == 0 || level.height == 0 {
            panic!("Level dimensions must be set before specifying map layout");
        }

        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => level.tiles.push(TileType::Wall),
                ' ' => level.tiles.push(TileType::Floor),
                '.' => {
                    level.targets.push(Point2D {
                        x: i as i32,
                        y: *row,
                    });
                    level.tiles.push(TileType::Target);
                }
                '@' => {
                    level.player = Point2D {
                        x: i as i32,
                        y: *row,
                    };
                    level.tiles.push(TileType::Floor);
                }
                '$' => {
                    level.boxes.push(Point2D {
                        x: i as i32,
                        y: *row,
                    });
                    level.tiles.push(TileType::Floor);
                }
                _ => (),
            }
        }

        if (*row + 1) < level.height {
            *row += 1;
        } else {
            *row = 0;
        }
    }

    pub fn get_level(&self, idx: usize) -> Option<&Level> {
        self.levels.get(idx)
    }
}
