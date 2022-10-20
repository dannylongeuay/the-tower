use std::collections::HashMap;

use crate::position::Position;

use super::{grid::Grid, tile::Tile};

pub struct Tower {
    levels: HashMap<&'static str, Level>,
}

impl Tower {
    pub fn new() -> Self {
        let mut levels: HashMap<&str, Level> = HashMap::new();
        Tower { levels }
    }
    pub fn get_tile(&self, level: &str, map: &str, pos: &Position) -> Option<&Tile> {
        if let Some(level) = self.levels.get(level) {
            if let Some(map) = level.maps.get(map) {
                return map.grid.get(pos);
            }
        }
        None
    }
    pub fn get_tile_mut(&mut self, level: &str, map: &str, pos: &Position) -> Option<&mut Tile> {
        if let Some(level) = self.levels.get_mut(level) {
            if let Some(map) = level.maps.get_mut(map) {
                return map.grid.get_mut(pos);
            }
        }
        None
    }
    pub fn insert_map_from_str(
        &mut self,
        level_name: &'static str,
        map_name: &'static str,
        width: usize,
        height: usize,
        s: &str,
    ) {
        if let Some(level) = self.levels.get_mut(level_name) {
            level
                .maps
                .insert(map_name, Map::from_str(map_name, width, height, s));
        } else {
            let mut maps: HashMap<&'static str, Map> = HashMap::new();
            maps.insert(map_name, Map::from_str(map_name, width, height, s));
            self.levels.insert(
                level_name,
                Level {
                    name: level_name,
                    maps,
                },
            );
        }
    }
}

struct Level {
    name: &'static str,
    maps: HashMap<&'static str, Map>,
}

struct Map {
    name: &'static str,
    grid: Grid<Tile>,
}

impl Map {
    fn from_str(name: &'static str, width: usize, height: usize, s: &str) -> Self {
        let mut grid: Grid<Tile> = Grid::new(width, height);
        let mut y = 0;
        for row in s.lines().map(str::trim).rev() {
            if !row.starts_with("|") {
                continue;
            }
            let mut x = 0;
            let splits: Vec<&str> = row.trim_start_matches("|").split_terminator("|").collect();
            for split in splits {
                let pos = Position { x, y };
                let chars: Vec<char> = split.chars().collect();
                match chars[0] {
                    'W' => {
                        if let Some(tile) = grid.get_mut(&pos) {
                            tile.walkable = false;
                            tile.opaque = false;
                        }
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }

        Map { name, grid }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_map_from_str() {
        let map = Map::from_str(
            "test_map",
            5,
            5,
            "
        | | | | | |
        | | | | | |
        | |W| | | |
        | | | |W| |
        | | | | | |
            ",
        );
        assert!(!map.grid.get(&Position { x: 1, y: 2 }).unwrap().walkable);
        assert!(map.grid.get(&Position { x: 2, y: 2 }).unwrap().walkable);
        assert!(!map.grid.get(&Position { x: 3, y: 1 }).unwrap().walkable);
    }
}
