use bevy::prelude::*;

use assets::AssetsPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use position::PositionPlugin;
use settings::SettingsPlugin;
use tower::TowerPlugin;

mod assets;
mod camera;
mod constants;
// mod hostiles;
mod player;
mod position;
mod settings;
mod tower;

fn main() {
    App::new()
        .add_plugin(SettingsPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PositionPlugin)
        .add_plugin(TowerPlugin)
        .run();
}
