use bevy::{prelude::*, render::texture::ImageSettings};

const SPRITE_SCALE: Vec3 = Vec3::new(5., 5., 1.);
const PLAYER_SPRITE_INDEX: usize = 4;

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
        .add_startup_system_to_stage(StartupStage::PostStartup, display_player_system)
        .run();
}

#[derive(Component)]
struct Player;

struct TextureHandles {
    atlas: Handle<TextureAtlas>,
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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

fn display_player_system(mut commands: Commands, texture_handles: Res<TextureHandles>) {
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
