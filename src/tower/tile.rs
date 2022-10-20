use bevy::prelude::*;

pub struct Tile {
    pub walkable: bool,
    pub opaque: bool,
    pub entities: Vec<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            walkable: true,
            opaque: true,
            entities: Vec::new(),
        }
    }
}
