#[macro_use]
extern crate derive_error;

mod resources;
mod systems;

use bevy::prelude::*;

fn main() {
    App::new()
        .init_resource::<resources::Player>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup)
        .add_system(systems::move_player)
        .run();
}
