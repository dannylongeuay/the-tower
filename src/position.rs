use bevy::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position_system);
    }
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub fn distance2(&self, other: &Self) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f32
    }
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance2(other).sqrt()
    }
}

fn update_position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut tf, pos) in query.iter_mut() {
        tf.translation.x = (pos.x * TILE_WIDTH) as f32;
        tf.translation.y = (pos.y * TILE_HEIGHT) as f32;
    }
}
