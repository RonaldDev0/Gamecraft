use bevy::prelude::*;

use crate::player::{
    camera::spawn_player_camera,
    movement::{player_movement, mouse_look}
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player_camera)
            .add_systems(Update, (player_movement, mouse_look));
    }
}
