use bevy::prelude::*;

#[derive(Debug)]
pub struct Tile {
    pub walkable: bool,
    pub transparent: bool,
    pub entities: Vec<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            walkable: true,
            transparent: true,
            entities: Vec::new(),
        }
    }
}
