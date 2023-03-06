mod bundles;
mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod util;

use bevy::{prelude::*, time::FixedTimestep};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use clap::Parser;

use components::{AmmoCount, Health, Poison, RateOfFire, Speed, Tick, TimeToLive};
use resources::{Config, SmallRng};

const TIME_STEP: f32 = 1.0 / 60.0; //rapier runs at 60fps by default.

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    InGame,
}

#[derive(Resource, Default)]
struct Sprites {
    handles: Vec<HandleUntyped>,
}

fn load_sprites(mut sprite_handles: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}

fn main() {
    let config = Config::parse();
    // group ui systems together bc we want to run them as fast as possible
    let ui_system_set = SystemSet::on_update(AppState::InGame)
        .with_system(systems::update_compass)
        .with_system(systems::update_score_ui)
        .with_system(systems::update_health_ui)
        .with_system(systems::sync_mouse_position)
        .with_system(systems::follow_player_with_camera)
        .with_system(systems::on_no_players_show_game_over)
        .with_system(systems::on_click_and_no_player_reset)
        .with_system(systems::update_sprite_index);

    let compute_powerups_system_set = SystemSet::on_update(AppState::InGame)
        .label("compute_powerups_system_set")
        .with_system(systems::powerup_defaulter::<Speed>)
        .with_system(systems::powerup_mather::<Speed>.after(systems::powerup_defaulter::<Speed>))
        .with_system(systems::powerup_defaulter::<Health>)
        .with_system(systems::powerup_mather::<Health>.after(systems::powerup_defaulter::<Health>))
        .with_system(systems::powerup_defaulter::<RateOfFire>)
        .with_system(
            systems::powerup_mather::<RateOfFire>.after(systems::powerup_defaulter::<RateOfFire>),
        )
        .with_system(systems::powerup_defaulter::<TimeToLive>)
        .with_system(
            systems::powerup_mather::<TimeToLive>.after(systems::powerup_defaulter::<TimeToLive>),
        )
        .with_system(systems::powerup_defaulter::<Poison>)
        .with_system(systems::powerup_mather::<Poison>.after(systems::powerup_defaulter::<Poison>))
        .with_system(systems::powerup_defaulter::<AmmoCount>)
        .with_system(
            systems::powerup_mather::<AmmoCount>.after(systems::powerup_defaulter::<AmmoCount>),
        );

    let game_loop_system_set = SystemSet::on_update(AppState::InGame)
        .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        // .with_system(systems::debug_time)
        .with_system(systems::count_ticks) //this may be off by one
        .with_system(systems::shoot_gun)
        .with_system(systems::move_bullet)
        .with_system(systems::spawn_mechs)
        .with_system(systems::spawn_mobs)
        .with_system(systems::chasers_follow_other_teams)
        .with_system(systems::player_aimables_aim_at_cursor)
        .with_system(systems::chaser_aimables_aim_at_other_teams)
        .with_system(systems::on_scroll_wheel_switch_ammo)
        .with_system(systems::on_f_key_switch_ammo)
        .with_system(systems::on_left_click_shoot)
        .with_system(systems::mob_shoot)
        .with_system(systems::move_player)
        .with_system(systems::move_thing)
        .with_system(systems::on_move_event_update_sprite_animation)
        .with_system(systems::on_move_event_update_3d_rotation)
        .with_system(systems::on_move_event_advance_3d_walking_animation)
        .with_system(systems::calculate_rate_of_fire)
        .with_system(systems::rotate_thing)
        // .with_system(systems::detect_exit)
        .with_system(systems::team_powerup_assigns_team)
        .with_system(systems::on_chest_hit_pickup)
        .with_system(systems::spawn_powerups)
        .with_system(systems::attach_time_to_live)
        .with_system(systems::time_to_live)
        .with_system(systems::on_0_health_kill)
        .with_system(systems::poison)
        .with_system(systems::attach_poison)
        .label("game_loop_system_set");

    let game_loop_cleanup_system_set =
        SystemSet::on_update(AppState::InGame).with_system(systems::consume_despawn_entity_events);

    let startup_system_set = SystemSet::on_enter(AppState::InGame)
        .with_system(load_sprites)
        .with_system(systems::spawn_camera)
        .with_system(systems::spawn_player)
        .with_system(systems::load_mech_walking_animation)
        .with_system(systems::spawn_ui)
        .with_system(systems::spawn_crosshairs)
        .with_system(systems::spawn_lights);

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
        .insert_resource(config)
        .insert_resource(Tick(0))
        .insert_resource(SmallRng::from_entropy())
        .init_resource::<Sprites>()
        .add_event::<events::MoveEvent>()
        .add_event::<events::RotateEvent>()
        .add_event::<events::DamagerHitEvent>()
        .add_event::<events::DespawnEvent>()
        .add_event::<events::ShootEvent>()
        .add_state(AppState::InGame)
        .add_startup_system(systems::resize_window)
        .add_system_set(startup_system_set)
        .add_system_set(ui_system_set)
        .add_system_set(compute_powerups_system_set)
        .add_system_set(game_loop_system_set.after("compute_powerups_system_set"))
        .add_system_set(game_loop_cleanup_system_set.after("game_loop_system_set"))
        .add_system_set(cleanup_system_set)
        .run();
}
