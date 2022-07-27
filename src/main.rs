#[macro_use]
extern crate derive_error;

mod bundles;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use heron::PhysicsPlugin;
use resources::MobSpawnTimer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(MobSpawnTimer(Timer::from_seconds(5.0, true)))
        .add_startup_system(systems::spawn_camera)
        .add_startup_system(systems::spawn_room)
        .add_startup_system(systems::spawn_player)
        .add_startup_system(systems::spawn_stick)
        .add_system(systems::spawn_mobs)
        .add_system(systems::move_player)
        .add_system(systems::update_carried_item_locations)
        .run();
}
