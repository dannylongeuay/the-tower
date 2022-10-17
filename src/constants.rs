use bevy::prelude::*;

// Settings
pub const DEFAULT_BACKGROUND_COLOR: ClearColor = ClearColor(Color::BLACK);
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.;

// Camera
pub const CAMERA_SCALE: f32 = 0.25;

// Tower
pub const TILE_WIDTH: i32 = 8;
pub const TILE_HEIGHT: i32 = 8;
pub const SPRITE_SCALE: Vec3 = Vec3::new(1., 1., 1.);
pub const PLAYER_SPRITE_INDEX: usize = 4;
pub const FLOOR_SPRITE_INDEX: usize = 97;
// pub const FLOOR_SPRITE_INDEX: usize = 17;

// Player
pub const SPRITE_COLOR_EXPLORED_VISIBLE: Color = Color::WHITE;
pub const SPRITE_COLOR_EXPLORED_SHROUDED: Color = Color::rgba(1., 1., 1., 0.25);
