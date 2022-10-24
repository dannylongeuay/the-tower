use bevy::prelude::*;

use crate::{
    constants::{CAMERA_SCALE, TILE_HEIGHT, TILE_WIDTH},
    player::Player,
    position::Position,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera_system)
            .add_system(update_camera_system);
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..default()
        },
        ..default()
    });
}

fn update_camera_system(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let pos = player_query.single();
    let mut tf = camera_query.single_mut();
    tf.translation.x = (pos.x * TILE_WIDTH) as f32;
    tf.translation.y = (pos.y * TILE_HEIGHT) as f32;
}
