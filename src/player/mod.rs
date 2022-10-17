use bevy::prelude::*;

use crate::{
    constants::{SPRITE_COLOR_EXPLORED_SHROUDED, SPRITE_COLOR_EXPLORED_VISIBLE},
    position::Position,
    tower::Explorable,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_system)
            .add_system(update_visiblity_system)
            .add_system(update_explorable_system);
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
            sprite.color = SPRITE_COLOR_EXPLORED_VISIBLE;
        } else {
            sprite.color = SPRITE_COLOR_EXPLORED_SHROUDED;
        }
    }
}
