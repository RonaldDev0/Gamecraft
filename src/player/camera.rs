use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera,
    ));
}
