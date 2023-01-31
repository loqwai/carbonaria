mod bundles;
mod components;
mod events;
mod resources;
mod systems;
mod util;

use bevy::{prelude::*, time::FixedTimestep};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use components::Tick;
use resources::{Config, SmallRng};

const TIME_STEP: f32 = 1.0 / 60.0; //rapier runs at 60fps by default.

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    InGame,
}

fn main() {
    // group ui systems together bc we want to run them as fast as possible
    let ui_system_set = SystemSet::on_update(AppState::InGame)
        .with_system(systems::update_compass)
        .with_system(systems::update_score_ui)
        .with_system(systems::update_health_ui)
        .with_system(systems::sync_mouse_position)
        .with_system(systems::follow_player_with_camera)
        .with_system(systems::on_no_players_show_game_over)
        .with_system(systems::on_click_and_no_player_reset);

    let game_loop_system_set = SystemSet::on_update(AppState::InGame)
        //https://bevy-cheatbook.github.io/programming/run-criteria.html
        .with_run_criteria(FixedTimestep::step(TIME_STEP as f64).with_label("foo"))
        .with_system(systems::debug_time)
        .with_system(systems::count_ticks) //this may be off by one
        .with_system(systems::shoot_gun)
        .with_system(systems::move_bullet)
        .with_system(systems::spawn_exit)
        .with_system(systems::spawn_mobs)
        .with_system(systems::chasers_follow_other_teams)
        .with_system(systems::player_aimables_aim_at_cursor)
        .with_system(systems::chaser_aimables_aim_at_other_teams)
        .with_system(systems::move_player)
        .with_system(systems::move_thing)
        .with_system(systems::rotate_thing)
        .with_system(systems::detect_exit)
        .with_system(systems::detect_damager_hits)
        .with_system(systems::team_powerup_assigns_team)
        .with_system(systems::on_0_health_kill)
        .with_system(systems::on_chest_hit_pickup)
        .with_system(systems::spawn_powerups)
        .with_system(systems::on_damager_hit_subtract_health)
        .with_system(systems::time_to_live);

    let startup_system_set = SystemSet::on_enter(AppState::InGame)
        .with_system(systems::spawn_camera)
        .with_system(systems::spawn_player)
        .with_system(systems::spawn_ui)
        .with_system(systems::spawn_crosshairs);

    let cleanup_system_set =
        SystemSet::on_exit(AppState::InGame).with_system(systems::remove_all_entities);

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default()) // the physics debug UI
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(Config {
            dimensions: 32,
            tile_size: 64,
            scale: 0.25, // 1.0 means the player is 256x256 px
            camera_follow_interpolation: 0.05,
            mob_spawn_interval: 100,
            powerup_spawn_interval: 100,
        })
        .insert_resource(Tick(0))
        .insert_resource(SmallRng::from_entropy())
        .add_event::<events::MoveEvent>()
        .add_event::<events::RotateEvent>()
        .add_event::<events::DamagerHitEvent>()
        .add_state(AppState::InGame)
        .add_startup_system(systems::resize_window)
        .add_system_set(startup_system_set)
        .add_system_set(ui_system_set)
        .add_system_set(game_loop_system_set)
        .add_system_set(cleanup_system_set)
        .run();
}
