use bevy::prelude::*;
use crate::voxel::{chunk::Chunk, types::{CHUNK_SIZE, GRASS, DIRT, STONE, SAND, WATER, WOOD, LEAVES, AIR}};

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub chunks: Vec<Chunk>
}

impl ChunkManager {
    pub fn generate_test_chunk(&mut self) {
        let mut c = Chunk::new(IVec3::new(0,0,0));

        // Fill a more interesting terrain
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if y < 2 {
                        c.set_block(x, y, z, STONE);
                    } else if y < 4 {
                        c.set_block(x, y, z, DIRT);
                    } else if y == 4 {
                        c.set_block(x, y, z, GRASS);
                    } else if y == 5 && x > 8 && x < 12 && z > 8 && z < 12 {
                        // Add some trees
                        c.set_block(x, y, z, WOOD);
                    } else if y == 6 && x > 7 && x < 13 && z > 7 && z < 13 {
                        // Tree leaves
                        c.set_block(x, y, z, LEAVES);
                    } else if y == 4 && x > 2 && x < 6 && z > 2 && z < 6 {
                        // Add some sand
                        c.set_block(x, y, z, SAND);
                    } else if y == 3 && x > 2 && x < 6 && z > 2 && z < 6 {
                        // Water under sand
                        c.set_block(x, y, z, WATER);
                    } else {
                        c.set_block(x, y, z, AIR);
                    }
                }
            }
        }

        self.chunks.push(c);
    }
}