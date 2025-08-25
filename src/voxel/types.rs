use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub id: u8,
    pub name: &'static str,
    pub color: Color,
}

impl Block {
    pub const fn new(id: u8, name: &'static str, color: Color) -> Self {
        Self { id, name, color }
    }
}

// Predefined block types
pub const AIR: Block = Block::new(0, "Air", Color::srgba(0.0, 0.0, 0.0, 0.0));
pub const GRASS: Block = Block::new(1, "Grass", Color::srgba(0.2, 0.8, 0.2, 1.0));
pub const DIRT: Block = Block::new(2, "Dirt", Color::srgba(0.6, 0.4, 0.2, 1.0));
pub const STONE: Block = Block::new(3, "Stone", Color::srgba(0.5, 0.5, 0.5, 1.0));
pub const SAND: Block = Block::new(4, "Sand", Color::srgba(0.76, 0.7, 0.5, 1.0));
pub const WATER: Block = Block::new(5, "Water", Color::srgba(0.0, 0.5, 0.8, 0.7));
pub const WOOD: Block = Block::new(6, "Wood", Color::srgba(0.6, 0.4, 0.2, 1.0));
pub const LEAVES: Block = Block::new(7, "Leaves", Color::srgba(0.1, 0.6, 0.1, 1.0));

// Block registry for easy lookup
// pub fn get_block_by_id(id: u8) -> Option<Block> {
//     match id {
//         0 => Some(AIR),
//         1 => Some(GRASS),
//         2 => Some(DIRT),
//         3 => Some(STONE),
//         4 => Some(SAND),
//         5 => Some(WATER),
//         6 => Some(WOOD),
//         7 => Some(LEAVES),
//         _ => None,
//     }
// }

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;