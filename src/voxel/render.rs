use bevy::prelude::*;
use crate::voxel::{manager::ChunkManager, mesher::generate_mesh_for_chunk};

pub fn spawn_chunk_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cm: ResMut<ChunkManager>,
) {
    // Ensure we have at least one chunk
    if cm.chunks.is_empty() {
        cm.generate_test_chunk();
    }

    // For each chunk, generate mesh (asynchronous for now)
    for (_i, chunk) in cm.chunks.iter().enumerate() {
        let chunkmesh = generate_mesh_for_chunk(chunk);
        let mesh_handle = meshes.add(chunkmesh.mesh);

        // Simple material
        let mat_handle = materials.add(Color::srgb(0.8, 0.7, 0.6));

        // Spawn entity with Mesh
        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(mat_handle),
            Transform::from_translation(Vec3::ZERO),
            Visibility::default(),
        ));
    }
}