use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets_system);
    }
}

pub struct TextureHandles {
    pub atlas: Handle<TextureAtlas>,
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
