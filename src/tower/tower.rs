use std::collections::{HashMap, HashSet};

use crate::position::{Position, DIRECTIONS};

use super::{grid::Grid, tile::Tile};

pub struct Tower {
    levels: HashMap<&'static str, Level>,
}

impl Tower {
    pub fn new() -> Self {
        let levels: HashMap<&str, Level> = HashMap::new();
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
    pub fn insert_empty_map(
        &mut self,
        level_name: &'static str,
        map_name: &'static str,
        width: usize,
        height: usize,
    ) {
        if let Some(level) = self.levels.get_mut(level_name) {
            level
                .maps
                .insert(map_name, Map::new(map_name, width, height));
        } else {
            let mut maps: HashMap<&'static str, Map> = HashMap::new();
            maps.insert(map_name, Map::new(map_name, width, height));
            self.levels.insert(
                level_name,
                Level {
                    name: level_name,
                    maps,
                },
            );
        }
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
    pub fn get_visible(
        &self,
        level_name: &'static str,
        map_name: &'static str,
        origin: &Position,
    ) -> HashSet<Position> {
        if let Some(level) = self.levels.get(level_name) {
            if let Some(map) = level.maps.get(map_name) {
                return map.get_visible(origin);
            }
        }
        HashSet::new()
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
    fn new(name: &'static str, width: usize, height: usize) -> Self {
        let grid: Grid<Tile> = Grid::new(width, height);
        Map { name, grid }
    }
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
                            tile.transparent = false;
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
    fn get_visible(&self, origin: &Position) -> HashSet<Position> {
        let mut visible: HashSet<Position> = HashSet::new();
        visible.insert(*origin);
        for dir in DIRECTIONS {
            let row = Row::new(1., -1., 1.);
            self.scan(row, &mut visible, origin, dir);
        }
        visible
    }
    fn scan(&self, mut row: Row, vis: &mut HashSet<Position>, origin: &Position, dir: &Position) {
        let mut prev: Option<&Tile> = None;
        for relative_position in row.relative_positions() {
            let absolute_pos = origin.quadrant_transform(dir, &relative_position);
            let tile_option = self.grid.get(&absolute_pos);
            if tile_option.is_none() {
                continue;
            }
            let tile = tile_option.unwrap();
            if !tile.transparent || row.symmetric(&relative_position) {
                vis.insert(absolute_pos);
            }
            if let Some(prev_tile) = prev {
                if !prev_tile.transparent && tile.transparent {
                    row.start = relative_position.slope();
                }
                if prev_tile.transparent && !tile.transparent {
                    let mut next_row = row.next();
                    next_row.end = relative_position.slope();
                    self.scan(next_row, vis, origin, dir);
                }
            }
            prev = Some(tile);
        }
        if let Some(prev_tile) = prev {
            if prev_tile.transparent {
                self.scan(row.next(), vis, origin, dir);
            }
        }
    }
}

struct Row {
    depth: f32,
    start: f32,
    end: f32,
}

impl Row {
    fn new(depth: f32, start: f32, end: f32) -> Self {
        Row { depth, start, end }
    }
    fn relative_positions(&self) -> Vec<Position> {
        let min = (self.depth * self.start + 0.5).floor() as i32;
        let max = (self.depth * self.end - 0.5).ceil() as i32;
        let mut result: Vec<Position> = Vec::new();
        for i in min..=max {
            let pos = Position::new(self.depth as i32, i);
            result.push(pos);
        }
        result
    }
    fn symmetric(&self, pos: &Position) -> bool {
        pos.y as f32 >= self.depth * self.start && pos.y as f32 <= self.depth * self.end
    }
    fn next(&self) -> Self {
        Row {
            depth: self.depth + 1.,
            start: self.start,
            end: self.end,
        }
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

    #[test]
    fn test_get_visible() {
        let map = Map::from_str(
            "test_map",
            5,
            5,
            "
        | | | | | |
        | |W|W|W| |
        | |W| | | |
        | |W|W|W| |
        | | | | | |
            ",
        );
        let visible = map.get_visible(&Position::new(2, 2));
        assert_eq!(visible.len(), 12);
    }
}
