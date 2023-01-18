#[macro_use]
extern crate derive_error;

mod bundles;
mod components;
mod events;
mod resources;
mod systems;
mod util;
mod inspection_ui;

use bevy::{prelude::*, render::texture::ImageSettings};

use heron::PhysicsPlugin;
use rand::{rngs::SmallRng, SeedableRng};
use resources::{Config, MobSpawnTimer};

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(MobSpawnTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(Config {
            dimensions: 128,
            tile_size: 64,
            camera_follow_interpolation: 0.05,
            // camera_follow_interpolation: 1.0,
        })
        .insert_resource(SmallRng::from_entropy())
        .insert_resource(ImageSettings::default_nearest())
        .add_event::<events::SwingStickEvent>()
        .add_event::<events::StickHitEvent>()
        .add_event::<events::ResetEvent>()
        .add_event::<events::MoveEvent>()
        .add_startup_system(systems::resize_window)
        .add_startup_system(systems::spawn_camera)
        .add_startup_system(systems::spawn_player)
        .add_startup_system(systems::spawn_score_ui)
        .add_startup_system(systems::spawn_speed_chest)
        .add_startup_system(systems::spawn_wallbreaker_chest)
        .add_startup_system(systems::spawn_powerups)
        // .add_system(systems::spawn_room)
        .add_system(systems::spawn_exit)
        .add_system(systems::spawn_next_tile_for_rooms)
        .add_system(systems::spawn_mobs)
        .add_system(systems::chasers_follow_other_teams)
        .add_system(systems::move_player)
        .add_system(systems::move_thing)
        .add_system(systems::follow_player_with_camera)
        .add_system(systems::on_click_swing_stick)
        .add_system(systems::mob_swings_stick_if_player_gets_close)
        .add_system(systems::begin_swing_stick_animation)
        .add_system(systems::maybe_end_swing_stick_animation)
        .add_system(systems::detect_stick_hits)
        .add_system(systems::detect_exit)
        .add_system(systems::team_powerup_assigns_team)
        .add_system(systems::on_0_health_kill)
        .add_system(systems::on_chest_hit_pickup)
        .add_system(systems::on_stick_hit_increment_points)
        .add_system(systems::on_reset_move_player_to_origin)
        .add_system(systems::on_stick_hit_subtract_health)
        .add_system(systems::update_compass)
        .add_system(systems::update_score_ui)
        .add_system(systems::update_health_ui)
        .add_system(systems::round_translations)
        .add_system(systems::on_reset_despawn_all_mobs)
        .add_system(systems::on_reset_despawn_all_rooms)
        .add_system(systems::on_reset_despawn_all_exits)
        .add_system(systems::on_stick_hit_wallbreaker)
        .add_system(systems::sync_mouse_position)
        ;

        inspection_ui::add_inspector(&mut app);
        app.run();
}
