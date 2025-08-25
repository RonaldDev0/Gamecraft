use bevy::prelude::*;

use crate::voxel::manager::ChunkManager;
use crate::voxel::render::spawn_chunk_meshes;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkManager::default())
            .add_systems(Startup, (
                spawn_chunk_meshes,
                spawn_light
            ));
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));
}