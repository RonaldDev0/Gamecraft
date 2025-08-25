use bevy::{
    asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, Mesh, PrimitiveTopology}
};

use crate::voxel::{
    chunk::Chunk,
    types::{CHUNK_SIZE, CHUNK_VOLUME, AIR}
};

pub struct ChunkMesh {
    pub mesh: Mesh,
    // pub translation: Vec3
}

// Ultra-optimized color conversion - converts Bevy Color to RGBA array
#[inline]
fn color_to_rgba_fast(color: &Color) -> [f32; 4] {
    let srgba = color.to_srgba();
    [srgba.red, srgba.green, srgba.blue, srgba.alpha]
}

fn push_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<[f32; 4]>,
    base_index: u32,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    d: [f32; 3],
    normal: [f32; 3],
    block_color: [f32; 4],
) {
    positions.push(a);
    positions.push(b);
    positions.push(c);
    positions.push(d);

    normals.push(normal);
    normals.push(normal);
    normals.push(normal);
    normals.push(normal);

    // UV coordinates for each vertex
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 1.0]);

    // Add colors for all 4 vertices of this quad
    colors.push(block_color);
    colors.push(block_color);
    colors.push(block_color);
    colors.push(block_color);

    // Correct winding order for CCW (counter-clockwise) when viewed from outside
    // First triangle: a -> b -> c
    indices.push(base_index + 0);
    indices.push(base_index + 1);
    indices.push(base_index + 2);

    // Second triangle: a -> c -> d
    indices.push(base_index + 0);
    indices.push(base_index + 2);
    indices.push(base_index + 3);
}

fn should_render_face(chunk: &Chunk, x: i32, y: i32, z: i32) -> bool {
    // Check bounds first
    if x < 0 || y < 0 || z < 0 || 
       x >= CHUNK_SIZE as i32 || y >= CHUNK_SIZE as i32 || z >= CHUNK_SIZE as i32 {
        return true; // Render face at chunk boundary
    }
    
    // Check if adjacent block is air
    chunk.get_block(x as usize, y as usize, z as usize).id == AIR.id
}

pub fn generate_mesh_for_chunk(chunk: &Chunk) -> ChunkMesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    // Pre-allocate vectors for better performance
    positions.reserve(CHUNK_VOLUME * 6 * 4); // 6 faces * 4 vertices per face, worst case
    normals.reserve(CHUNK_VOLUME * 6 * 4);
    uvs.reserve(CHUNK_VOLUME * 6 * 4);
    colors.reserve(CHUNK_VOLUME * 6 * 4);
    indices.reserve(CHUNK_VOLUME * 6 * 6); // 6 faces * 6 indices per face

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let block = chunk.get_block(x, y, z);
                if block.id == AIR.id { continue; }

                let world_x = x as f32 + (chunk.position.x as f32 * CHUNK_SIZE as f32);
                let world_y = y as f32 + (chunk.position.y as f32 * CHUNK_SIZE as f32);
                let world_z = z as f32 + (chunk.position.z as f32 * CHUNK_SIZE as f32);

                let block_color = color_to_rgba_fast(&block.color);

                // Right face (+X) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32 + 1, y as i32, z as i32) {
                    let base_index = positions.len() as u32;
                    let a = [world_x + 1.0, world_y,       world_z + 1.0]; // bottom-right
                    let b = [world_x + 1.0, world_y,       world_z      ]; // bottom-left  
                    let c = [world_x + 1.0, world_y + 1.0, world_z      ]; // top-left
                    let d = [world_x + 1.0, world_y + 1.0, world_z + 1.0]; // top-right
                    let normal = [1.0, 0.0, 0.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }
                
                // Left face (-X) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32 - 1, y as i32, z as i32) {
                    let base_index = positions.len() as u32;
                    let a = [world_x, world_y,       world_z      ]; // bottom-right
                    let b = [world_x, world_y,       world_z + 1.0]; // bottom-left
                    let c = [world_x, world_y + 1.0, world_z + 1.0]; // top-left  
                    let d = [world_x, world_y + 1.0, world_z      ]; // top-right
                    let normal = [-1.0, 0.0, 0.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }

                // Top face (+Y) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32, y as i32 + 1, z as i32) {
                    let base_index = positions.len() as u32;
                    let a = [world_x,       world_y + 1.0, world_z + 1.0]; // bottom-left
                    let b = [world_x + 1.0, world_y + 1.0, world_z + 1.0]; // bottom-right
                    let c = [world_x + 1.0, world_y + 1.0, world_z      ]; // top-right
                    let d = [world_x,       world_y + 1.0, world_z      ]; // top-left
                    let normal = [0.0, 1.0, 0.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }

                // Bottom face (-Y) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32, y as i32 - 1, z as i32) {
                    let base_index = positions.len() as u32;
                    let a = [world_x,       world_y, world_z      ]; // bottom-left
                    let b = [world_x + 1.0, world_y, world_z      ]; // bottom-right
                    let c = [world_x + 1.0, world_y, world_z + 1.0]; // top-right
                    let d = [world_x,       world_y, world_z + 1.0]; // top-left
                    let normal = [0.0, -1.0, 0.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }

                // Front face (+Z) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32, y as i32, z as i32 + 1) {
                    let base_index = positions.len() as u32;
                    let a = [world_x,       world_y,       world_z + 1.0]; // bottom-left
                    let b = [world_x + 1.0, world_y,       world_z + 1.0]; // bottom-right
                    let c = [world_x + 1.0, world_y + 1.0, world_z + 1.0]; // top-right
                    let d = [world_x,       world_y + 1.0, world_z + 1.0]; // top-left
                    let normal = [0.0, 0.0, 1.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }

                // Back face (-Z) - vista desde afuera del cubo
                if should_render_face(chunk, x as i32, y as i32, z as i32 - 1) {
                    let base_index = positions.len() as u32;
                    let a = [world_x + 1.0, world_y,       world_z]; // bottom-left
                    let b = [world_x,       world_y,       world_z]; // bottom-right
                    let c = [world_x,       world_y + 1.0, world_z]; // top-right
                    let d = [world_x + 1.0, world_y + 1.0, world_z]; // top-left
                    let normal = [0.0, 0.0, -1.0];
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, &mut colors, 
                             base_index, a, b, c, d, normal, block_color);
                }
            }
        }
    }

    // Build Mesh - ultra-optimized
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    ChunkMesh { mesh }
}