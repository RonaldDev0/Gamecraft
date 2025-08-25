use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::player::camera::PlayerCamera;

const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 0.0005;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let forward = transform.forward().normalize();
        let right = transform.right().normalize();

        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= right;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += right;
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            direction -= Vec3::Y;
        }
        if keyboard.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * SPEED * time.delta_secs();
        }
    }
}

pub fn mouse_look(
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut delta = Vec2::ZERO;
        for event in mouse_events.read() {
            delta += event.delta;
        }

        if delta.length_squared() > 0.0 {
            let yaw = Quat::from_rotation_y(-delta.x * SENSITIVITY);
            let pitch = Quat::from_rotation_x(-delta.y * SENSITIVITY);

            // apply yaw (global Y axis)
            transform.rotation = yaw * transform.rotation;

            // apply pitch (local X axis)
            transform.rotation *= pitch;
        }
    }
}
