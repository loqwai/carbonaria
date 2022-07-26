#[macro_use]
extern crate derive_error;

mod bundles;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use heron::PhysicsPlugin;

fn main() {
    App::new()
        .init_resource::<resources::PlayerResource>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(systems::spawn_camera)
        .add_startup_system(systems::spawn_room)
        .add_startup_system(systems::spawn_player)
        .add_system(systems::move_player)
        .run();
}
