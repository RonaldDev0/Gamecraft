use bevy::prelude::*;

use crate::voxel::manager::ChunkManager;
use crate::voxel::render::spawn_chunk_meshes;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkManager::default())
            .add_systems(Startup, spawn_chunk_meshes);
    }
}