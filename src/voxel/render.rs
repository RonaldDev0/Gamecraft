use bevy::prelude::*;
use crate::voxel::{
    manager::ChunkManager,
    mesher::generate_mesh_for_chunk,
    types::CHUNK_SIZE
};

pub fn spawn_chunk_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cm: ResMut<ChunkManager>,
) {
    if cm.chunks.is_empty() {
        cm.generate_test_chunk();
    }

    for chunk in cm.chunks.iter() {
        let chunkmesh = generate_mesh_for_chunk(chunk);
        let mesh_handle = meshes.add(chunkmesh.mesh);

        let mat_handle = materials.add(Color::srgb(0.8, 0.7, 0.6));

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(mat_handle),
            Transform::from_translation(chunk.position.as_vec3() * CHUNK_SIZE as f32),
            Visibility::default(),
        ));
    }
}
