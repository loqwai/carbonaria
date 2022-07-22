#[macro_use]
extern crate derive_error;

mod bundles;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;

fn main() {
    App::new()
        .init_resource::<resources::PlayerResource>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::spawn_room)
        .add_startup_system(systems::spawn_player)
        .add_system(systems::move_player)
        .add_system(systems::detect_player_wall_collisions)
        .run();
}
