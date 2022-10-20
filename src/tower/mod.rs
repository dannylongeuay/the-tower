use bevy::prelude::*;

use crate::{
    assets::TextureHandles,
    constants::{FLOOR_SPRITE_INDEX, PLAYER_SPRITE_INDEX, SPRITE_SCALE},
    player::Player,
    position::Position,
};

use self::tile::Tile;
use self::{grid::Grid, tower::Tower};

mod grid;
mod map_gen;
mod tile;
pub mod tower;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tower_system);
    }
}

#[derive(Component)]
pub struct Explorable;

fn spawn_player_system(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: PLAYER_SPRITE_INDEX,
                ..default()
            },
            texture_atlas: texture_handles.atlas.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        })
        .insert(Position::ZERO)
        .insert(Player);
}

fn spawn_tower_system(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    let mut grid: Grid<Tile> = Grid::new(20, 20);
    for x in 0..=20 {
        for y in 0..=20 {
            let pos = Position { x, y };
            let mut ec = commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: FLOOR_SPRITE_INDEX,
                    ..default()
                },
                texture_atlas: texture_handles.atlas.clone(),
                transform: Transform {
                    scale: SPRITE_SCALE,
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                ..default()
            });
            if let Some(tile) = grid.get_mut(&pos) {
                tile.entities.push(ec.id())
            }
            ec.insert(pos);
            ec.insert(Explorable);
        }
    }
    let tower = Tower::new();
    commands.insert_resource(tower);
}
