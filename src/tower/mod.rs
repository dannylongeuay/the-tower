use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::TextureHandles,
    constants::{
        DIRT_SPRITE_INDEX, FLOOR_SPRITE_INDEX, GRASS_SPRITE_INDEX, PLAYER_SPRITE_INDEX,
        SPRITE_SCALE, WATER_SPRITE_INDEX,
    },
    player::Player,
    position::Position,
};

use self::map_gen::wfc::WFC;
use self::tower::Tower;

mod grid;
mod map_gen;
mod tile;
pub mod tower;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tower_system)
            .add_system(update_sprite_index_system);
    }
}

#[derive(Component)]
pub struct Tile;

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
        .insert(Position::new(10, 10))
        .insert(Player);
}

fn spawn_tower_system(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    let width = 20;
    let height = 20;
    let mut tower = Tower::new();
    let level_name = "level1";
    let map_name = "map1";
    tower.insert_empty_map(level_name, map_name, width, height);
    let constraints: Vec<Vec<(Position, Vec<usize>)>> = vec![
        vec![
            (Position::UP, vec![0, 1]),
            (Position::DOWN, vec![0, 1]),
            (Position::LEFT, vec![0, 1]),
            (Position::RIGHT, vec![0, 1]),
        ],
        vec![
            (Position::UP, vec![0, 1, 2]),
            (Position::DOWN, vec![0, 1, 2]),
            (Position::LEFT, vec![0, 1, 2]),
            (Position::RIGHT, vec![0, 1, 2]),
        ],
        vec![
            (Position::UP, vec![1, 2]),
            (Position::DOWN, vec![1, 2]),
            (Position::LEFT, vec![1, 2]),
            (Position::RIGHT, vec![1, 2]),
        ],
    ];
    let wfc = WFC::new(width, height, constraints, vec![1, 1, 1]);
    commands.insert_resource(wfc);
    let wave_timer = WaveTimer {
        timer: Timer::new(Duration::from_millis(50), true),
    };
    commands.insert_resource(wave_timer);
    for y in 0..height {
        for x in 0..width {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
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
                visibility: Visibility { is_visible: true },
                ..default()
            });
            if let Some(tile) = tower.get_tile_mut(level_name, map_name, &pos) {
                tile.entities.push(ec.id())
            }
            ec.insert(pos);
            ec.insert(Explorable);
            ec.insert(Tile);
        }
    }
    commands.insert_resource(tower);
}

struct WaveTimer {
    timer: Timer,
}

fn update_sprite_index_system(
    mut wfc: ResMut<WFC>,
    time: Res<Time>,
    mut timer: ResMut<WaveTimer>,
    mut query: Query<(&mut TextureAtlasSprite, &Position), With<Tile>>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }
    if wfc.uncollapsed_cells == 0 {
        return;
    }
    wfc.next();
    for (mut sprite, pos) in query.iter_mut() {
        let index = match wfc.cells.get(&pos).unwrap().chosen_index {
            Some(0) => DIRT_SPRITE_INDEX,
            Some(1) => GRASS_SPRITE_INDEX,
            Some(2) => WATER_SPRITE_INDEX,
            _ => FLOOR_SPRITE_INDEX,
        };
        sprite.index = index;
    }
}
