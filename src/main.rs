mod bundles;
mod components;
mod events;
mod resources;
mod systems;
mod util;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use resources::{Config, MobSpawnTimer, SmallRng};

fn main() {
    let uiSystemSet = SystemSet::new()
        .with_system(systems::update_compass)
        .with_system(systems::update_score_ui)
        .with_system(systems::update_health_ui)
        .with_system(systems::sync_mouse_position)
        .with_system(systems::player_aimables_aim_at_cursor)
        .with_system(systems::follow_player_with_camera)
        ;

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default()) // the physics debug UI
        .add_plugin(WorldInspectorPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(MobSpawnTimer(Timer::from_seconds(
            10.0,
            TimerMode::Repeating,
        )))
        .insert_resource(Config {
            dimensions: 32,
            tile_size: 64,
            camera_follow_interpolation: 0.05,
            // camera_follow_interpolation: 1.0,
        })
        .insert_resource(SmallRng::from_entropy())
        .add_event::<events::ResetEvent>()
        .add_event::<events::MoveEvent>()
        .add_event::<events::RotateEvent>()
        .add_event::<events::DamagerHitEvent>()
        .add_startup_system(systems::resize_window)
        .add_startup_system(systems::spawn_camera)
        .add_startup_system(systems::spawn_player)
        .add_startup_system(systems::spawn_ui)
        .add_startup_system(systems::spawn_speed_chest)
        .add_startup_system(systems::spawn_wallbreaker_chest)
        .add_startup_system(systems::spawn_powerups)
        .add_startup_system(systems::spawn_crosshairs)
        .add_system_set(uiSystemSet)
        .add_system(systems::shoot_gun)
        .add_system(systems::move_bullet)
        .add_system(systems::spawn_exit)
        .add_system(systems::spawn_mobs)
        .add_system(systems::chasers_follow_other_teams)
        .add_system(systems::move_player)
        .add_system(systems::move_thing)
        .add_system(systems::rotate_thing)
        .add_system(systems::detect_exit)
        .add_system(systems::detect_damager_hits)
        .add_system(systems::team_powerup_assigns_team)
        .add_system(systems::on_0_health_kill)
        .add_system(systems::on_chest_hit_pickup)
        .add_system(systems::on_damager_hit_subtract_health)
        .run();
}
