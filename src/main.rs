#[macro_use]
extern crate derive_error;

mod bundles;
mod components;
mod events;
mod resources;
mod systems;
mod util;

use bevy::prelude::*;
use heron::PhysicsPlugin;
use rand::{rngs::SmallRng, SeedableRng};
use resources::{Config, MobSpawnTimer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(MobSpawnTimer(Timer::from_seconds(5.0, true)))
        .insert_resource(Config { dimensions: 16 })
        .insert_resource(SmallRng::from_entropy())
        .add_event::<events::SwingStickEvent>()
        .add_event::<events::StickHitEvent>()
        .add_startup_system(systems::spawn_camera)
        .add_startup_system(systems::spawn_room)
        .add_startup_system(systems::spawn_player)
        .add_startup_system(systems::spawn_score_ui)
        .add_system(systems::spawn_next_tile_for_rooms)
        .add_system(systems::spawn_mobs)
        .add_system(systems::move_player)
        .add_system(systems::on_click_swing_stick)
        .add_system(systems::mob_swings_stick_if_player_gets_close)
        .add_system(systems::begin_swing_stick_animation)
        .add_system(systems::maybe_end_swing_stick_animation)
        .add_system(systems::detect_stick_hits)
        .add_system(systems::on_stick_hit_increment_points)
        .add_system(systems::on_stick_hit_kill)
        .add_system(systems::update_score_ui)
        .run();
}
