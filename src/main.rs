use bevy::{prelude::*, render::texture::ImageSettings};

const SPRITE_WIDTH: f32 = 8.;
const SPRITE_HEIGHT: f32 = 8.;
const SPRITE_SCALE: Vec3 = Vec3::new(1., 1., 1.);
const PLAYER_SPRITE_INDEX: usize = 4;
const FLOOR_SPRITE_INDEX: usize = 97;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
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
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_floor_system)
        .add_system(input_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Tower {
    tiles: Vec<Position>,
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
        .insert(Player);
}

fn spawn_floor_system(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    for x in -16..16 {
        for y in -16..16 {
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: FLOOR_SPRITE_INDEX,
                    ..default()
                },
                texture_atlas: texture_handles.atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * SPRITE_WIDTH, y as f32 * SPRITE_HEIGHT, 0.),
                    scale: SPRITE_SCALE,
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn input_system(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut tf = query
        .get_single_mut()
        .expect("Error: could not find player");
    if keyboard_input.just_pressed(KeyCode::H) {
        tf.translation.x -= SPRITE_WIDTH;
    }
    if keyboard_input.just_pressed(KeyCode::L) {
        tf.translation.x += SPRITE_WIDTH;
    }
    if keyboard_input.just_pressed(KeyCode::J) {
        tf.translation.y -= SPRITE_HEIGHT;
    }
    if keyboard_input.just_pressed(KeyCode::K) {
        tf.translation.y += SPRITE_HEIGHT;
    }
}
