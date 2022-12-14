use std::ops::{Add, Mul, Sub};

use bevy::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position_system);
    }
}
pub const DIRECTIONS: &'static [Position] = &[
    Position::UP,
    Position::DOWN,
    Position::LEFT,
    Position::RIGHT,
];

#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const UP: Self = Self { x: 0, y: 1 };
    pub const DOWN: Self = Self { x: 0, y: -1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
    pub fn distance2(&self, other: &Self) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f32
    }
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance2(other).sqrt()
    }
    pub fn abs(&self) -> Self {
        Position {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        Neighbors::new(*self)
    }
    pub fn quadrant_transform(&self, dir: &Self, other: &Self) -> Self {
        match *dir {
            Position::UP => return Position::new(self.x + other.y, self.y + other.x),
            Position::DOWN => return Position::new(self.x + other.y, self.y - other.x),
            Position::LEFT => return Position::new(self.x - other.x, self.y + other.y),
            Position::RIGHT => return Position::new(self.x + other.x, self.y + other.y),
            _ => unreachable!("invalid direction"),
        }
    }
    pub fn slope(&self) -> f32 {
        (2. * self.y as f32 - 1.) / (2. * self.x as f32)
    }
}

struct Neighbors {
    origin: Position,
    index: usize,
}

impl Neighbors {
    fn new(origin: Position) -> Self {
        Neighbors { origin, index: 0 }
    }
}

impl Iterator for Neighbors {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= DIRECTIONS.len() {
            return None;
        }
        let neighbor = *DIRECTIONS.get(self.index).unwrap() + self.origin;
        self.index += 1;
        Some(neighbor)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Position;
    fn mul(self, scalar: i32) -> Self {
        Position {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn update_position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut tf, pos) in query.iter_mut() {
        tf.translation.x = (pos.x * TILE_WIDTH) as f32;
        tf.translation.y = (pos.y * TILE_HEIGHT) as f32;
    }
}
