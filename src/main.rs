use bevy::prelude::*;

mod setup;

use crate::setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .run();
}
