use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    assets::TextureHandles,
    constants::{FLOOR_SPRITE_INDEX, PLAYER_SPRITE_INDEX, SPRITE_SCALE},
    player::Player,
    position::Position,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tower_system);
    }
}

#[derive(Component)]
pub struct Explorable;

struct Tile {
    pos: Position,
    entities: Vec<Entity>,
}

struct Map {
    name: &'static str,
    tiles: HashMap<Position, Tile>,
}

struct Level {
    name: &'static str,
    maps: HashMap<&'static str, Map>,
}

pub struct Tower {
    levels: HashMap<&'static str, Level>,
}

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
    let mut tiles: HashMap<Position, Tile> = HashMap::new();
    for x in -16..16 {
        for y in -16..16 {
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
            ec.insert(pos);
            ec.insert(Explorable);
            let tile = Tile {
                pos,
                entities: vec![ec.id()],
            };
            tiles.insert(pos, tile);
        }
    }
    let map = Map {
        name: "map1",
        tiles,
    };
    let mut maps: HashMap<&str, Map> = HashMap::new();
    maps.insert(map.name, map);
    let level = Level {
        name: "level1",
        maps,
    };
    let mut levels: HashMap<&str, Level> = HashMap::new();
    levels.insert(level.name, level);
    let tower = Tower { levels };
    commands.insert_resource(tower);
}
