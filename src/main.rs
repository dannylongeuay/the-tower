use std::collections::HashMap;

use bevy::{prelude::*, render::texture::ImageSettings};

const SPRITE_WIDTH: i32 = 8;
const SPRITE_HEIGHT: i32 = 8;
const SPRITE_SCALE: Vec3 = Vec3::new(1., 1., 1.);
const PLAYER_SPRITE_INDEX: usize = 4;
// const FLOOR_SPRITE_INDEX: usize = 17;
const FLOOR_SPRITE_INDEX: usize = 97;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "The Tower".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system(load_assets_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tower_system)
        .add_system(input_system)
        .add_system(update_position_system)
        .add_system(update_visiblity_system)
        .add_system(update_explorable_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance2(&self, other: &Self) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f32
    }
    fn distance(&self, other: &Self) -> f32 {
        self.distance2(other).sqrt()
    }
}

#[derive(Component)]
struct Explorable;

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

struct Tower {
    levels: HashMap<&'static str, Level>,
}

struct TextureHandles {
    atlas: Handle<TextureAtlas>,
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.25,
            ..default()
        },
        ..default()
    });
}

fn load_assets_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 16, 10);
    let atlas = texture_atlases.add(texture_atlas);
    commands.insert_resource(TextureHandles { atlas });
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
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        })
        .insert(Position { x: 0, y: 0 })
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

fn update_visiblity_system(
    player_query: Query<&Position, With<Player>>,
    mut ent_query: Query<(&mut Visibility, &Position), (Without<Player>, Without<Explorable>)>,
) {
    let player_pos = player_query
        .get_single()
        .expect("Error: could not find player");
    for (mut vis, pos) in ent_query.iter_mut() {
        match player_pos.distance(pos) < 5. {
            true => vis.is_visible = true,
            false => vis.is_visible = false,
        };
    }
}

fn update_explorable_system(
    player_query: Query<&Position, With<Player>>,
    mut ent_query: Query<(&mut Visibility, &mut TextureAtlasSprite, &Position), With<Explorable>>,
) {
    let player_pos = player_query
        .get_single()
        .expect("Error: could not find player");
    for (mut vis, mut sprite, pos) in ent_query.iter_mut() {
        if player_pos.distance(pos) < 5. {
            vis.is_visible = true;
            sprite.color = Color::rgba(1., 1., 1., 1.);
        } else {
            sprite.color = Color::rgba(1., 1., 1., 0.25);
        }
    }
}

fn update_position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut tf, pos) in query.iter_mut() {
        tf.translation.x = (pos.x * SPRITE_WIDTH) as f32;
        tf.translation.y = (pos.y * SPRITE_HEIGHT) as f32;
    }
}

fn input_system(
    mut query: Query<&mut Position, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut pos = query
        .get_single_mut()
        .expect("Error: could not find player");
    if keyboard_input.just_pressed(KeyCode::H) {
        pos.x -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::L) {
        pos.x += 1;
    }
    if keyboard_input.just_pressed(KeyCode::J) {
        pos.y -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::K) {
        pos.y += 1;
    }
}
