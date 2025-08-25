use bevy::{
    asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, Mesh, PrimitiveTopology}
};

use crate::voxel::{
    chunk::Chunk,
    types::{CHUNK_SIZE, AIR}
};

pub struct ChunkMesh {
    pub mesh: Mesh,
    // pub translation: Vec3
}

fn push_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    base_index: u32,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    d: [f32; 3],
    normal: [f32; 3],
    uv00: [f32; 2],
    uv11: [f32; 2],
) {
    positions.push(a);
    positions.push(b);
    positions.push(c);
    positions.push(d);

    normals.push(normal);
    normals.push(normal);
    normals.push(normal);
    normals.push(normal);

    uvs.push([uv00[0], uv00[1]]);
    uvs.push([uv11[0], uv00[1]]);
    uvs.push([uv11[0], uv11[1]]);
    uvs.push([uv00[0], uv11[1]]);

    indices.push(base_index + 0);
    indices.push(base_index + 1);
    indices.push(base_index + 2);

    indices.push(base_index + 2);
    indices.push(base_index + 3);
    indices.push(base_index + 0);

}

pub fn generate_mesh_for_chunk(chunk: &Chunk) -> ChunkMesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let id = chunk.get_block(x, y, z);
                if id == AIR { continue; }

                let world_x = x as f32 + (chunk.position.x as f32 * CHUNK_SIZE as f32);
                let world_y = y as f32 + (chunk.position.y as f32 * CHUNK_SIZE as f32);
                let world_z = z as f32 + (chunk.position.z as f32 * CHUNK_SIZE as f32);

                let base = Vec3::new(world_x, world_y, world_z);

                // +X face
                let nx = x as i32 + 1;
                 if nx >= CHUNK_SIZE as i32 || chunk.get_block(nx as usize, y, z) == AIR {
                     let bx = base + Vec3::new(1.0, 0.0, 0.0);
                     let a = [bx.x, base.y, base.z];
                     let b = [bx.x, base.y+1.0, base.z];
                     let c = [bx.x, base.y+1.0, base.z+1.0];
                     let d = [bx.x, base.y, base.z+1.0];
                     let normal = [1.0, 0.0, 0.0];
                     let base_index = positions.len() as u32;
                     push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                 }
                 
                 // -X face
                let nx = x as i32 - 1;
                if nx < 0 || chunk.get_block(nx as usize, y, z) == AIR {
                    let bx = base;
                    let a = [bx.x, base.y, base.z];
                    let b = [bx.x, base.y, base.z+1.0];
                    let c = [bx.x, base.y+1.0, base.z+1.0];
                    let d = [bx.x, base.y+1.0, base.z];
                    let normal = [-1.0, 0.0, 0.0];
                    let base_index = positions.len() as u32;
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                }

                // +Y face (top)
                let ny = y as i32 + 1;
                if ny >= CHUNK_SIZE as i32 || chunk.get_block(x, ny as usize, z) == AIR {
                    let by = base + Vec3::new(0.0,1.0,0.0);
                    let a = [base.x, by.y, base.z];
                    let b = [base.x+1.0, by.y, base.z];
                    let c = [base.x+1.0, by.y, base.z+1.0];
                    let d = [base.x, by.y, base.z+1.0];
                    let normal = [0.0,1.0,0.0];
                    let base_index = positions.len() as u32;
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                }

                // -Y face (bottom)
                let ny = y as i32 - 1;
                if ny < 0 || chunk.get_block(x, ny as usize, z) == AIR {
                    let a = [base.x, base.y, base.z];
                    let b = [base.x, base.y, base.z+1.0];
                    let c = [base.x+1.0, base.y, base.z+1.0];
                    let d = [base.x+1.0, base.y, base.z];
                    let normal = [0.0,-1.0,0.0];
                    let base_index = positions.len() as u32;
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                }

                // +Z face (front)
                let nz = z as i32 + 1;
                if nz >= CHUNK_SIZE as i32 || chunk.get_block(x, y, nz as usize) == AIR {
                    let bz = base + Vec3::new(0.0,0.0,1.0);
                    let a = [base.x, base.y, bz.z];
                    let b = [base.x, base.y+1.0, bz.z];
                    let c = [base.x+1.0, base.y+1.0, bz.z];
                    let d = [base.x+1.0, base.y, bz.z];
                    let normal = [0.0,0.0,1.0];
                    let base_index = positions.len() as u32;
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                }

                // -Z face (back)
                let nz = z as i32 - 1;
                if nz < 0 || chunk.get_block(x, y, nz as usize) == AIR {
                    let a = [base.x, base.y, base.z];
                    let b = [base.x+1.0, base.y, base.z];
                    let c = [base.x+1.0, base.y+1.0, base.z];
                    let d = [base.x, base.y+1.0, base.z];
                    let normal = [0.0,0.0,-1.0];
                    let base_index = positions.len() as u32;
                    push_quad(&mut positions, &mut normals, &mut uvs, &mut indices, base_index, a,b,c,d, normal, [0.0,0.0], [1.0,1.0]);
                }
            }
        }
    }

    // Build Mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    // let translation = Vec3::ZERO;

    ChunkMesh {
        mesh,
        // translation
    }
}