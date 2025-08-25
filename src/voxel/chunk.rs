use crate::voxel::types::*;
use bevy::prelude::*;

#[derive(Clone)]
pub struct Chunk {
    pub blocks: Vec<BlockId>,
    pub position: IVec3
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            blocks: vec![AIR; CHUNK_VOLUME],
            position
        }
    }

    #[inline]
    fn index(x: usize, y: usize, z: usize) -> usize {
        x + CHUNK_SIZE * (y + CHUNK_SIZE * z)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, id: BlockId) {
        let idx = Self::index(x, y, z);
        self.blocks[idx] = id;
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockId {
        let idx = Self::index(x, y, z);
        self.blocks[idx]
    }

    // pub fn in_bounds(x: i32, y: i32, z: i32) -> bool {
    //     (0..CHUNK_SIZE as i32).contains(&x)
    //         && (0..CHUNK_SIZE as i32).contains(&y)
    //         && (0..CHUNK_SIZE as i32).contains(&z)
    // }
}