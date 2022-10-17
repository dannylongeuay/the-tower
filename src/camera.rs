use bevy::prelude::*;

use crate::constants::CAMERA_SCALE;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_system);
    }
}

fn camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..default()
        },
        ..default()
    });
}
