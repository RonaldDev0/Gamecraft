use bevy::prelude::*;

mod setup;
mod voxel;
mod player;

use crate::setup::SetupPlugin;
use crate::voxel::VoxelPlugin;

fn main() {
    App::new()
        .add_plugins((SetupPlugin, VoxelPlugin))
        .run();
}
