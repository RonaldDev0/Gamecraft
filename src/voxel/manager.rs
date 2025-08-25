use bevy::prelude::*;
use crate::voxel::{chunk::Chunk, types::CHUNK_SIZE};

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub chunks: Vec<Chunk>
}

impl ChunkManager {
    pub fn generate_test_chunk(&mut self) {
        let mut c = Chunk::new(IVec3::new(0,0,0));

        // Fill a simple flat terrain: y < 6 dirt
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if y < 4 {
                        c.set_block(x, y, z, 2);
                    } else if y == 4 {
                        c.set_block(x, y, z, 1);
                    } else {
                        c.set_block(x, y, z, 0);
                    }
                }
            }
        }

        self.chunks.push(c);
    }
}