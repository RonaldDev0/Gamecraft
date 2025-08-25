use bevy::prelude::*;

mod setup;
mod voxel;
mod player;

use crate::setup::SetupPlugin;
use crate::voxel::VoxelPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            SetupPlugin,
            VoxelPlugin,
            PlayerPlugin
        ))
        .run();
}
