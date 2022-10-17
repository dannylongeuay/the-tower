use bevy::{prelude::*, render::texture::ImageSettings};

use crate::constants::{DEFAULT_BACKGROUND_COLOR, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest())
            .insert_resource(DEFAULT_BACKGROUND_COLOR)
            .insert_resource(WindowDescriptor {
                title: "The Tower".to_string(),
                width: DEFAULT_WINDOW_WIDTH,
                height: DEFAULT_WINDOW_HEIGHT,
                resizable: false,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            });
    }
}
