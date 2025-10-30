mod bundles;
mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod util;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use clap::Parser;

use components::{AmmoCount, Health, Poison, RateOfFire, Speed, Tick, TimeToLive};
use resources::{Config, SmallRng};

const TIME_STEP: f32 = 1.0 / 60.0; //rapier runs at 60fps by default.

#[derive(Clone, Debug, Hash, Eq, PartialEq, States, Default)]
pub enum AppState {
    #[default]
    InGame,
}

#[derive(Resource, Default)]
struct Sprites {
    handles: Vec<UntypedHandle>,
}

fn load_sprites(mut sprite_handles: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    let handle = asset_server.load_folder("sprites");
    sprite_handles.handles = vec![handle.untyped()];
}

// System sets for ordering
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ComputePowerupsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameLoopSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameLoopCleanupSet;

fn main() {
    let config = Config::parse();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) // the physics debug UI
        // .add_plugins(WorldInspectorPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
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
        .init_state::<AppState>()
        // Startup systems
        .add_systems(Startup, systems::resize_window)
        // OnEnter(AppState::InGame) systems
        .add_systems(
            OnEnter(AppState::InGame),
            (
                load_sprites,
                systems::spawn_camera,
                systems::spawn_player,
                systems::load_mech_walking_animation,
                systems::spawn_ui,
                systems::spawn_crosshairs,
                systems::spawn_lights,
            ),
        )
        // OnExit(AppState::InGame) systems
        .add_systems(OnExit(AppState::InGame), systems::remove_all_entities)
        // UI systems - run every frame
        .add_systems(
            Update,
            (
                systems::update_compass,
                systems::update_score_ui,
                systems::update_health_ui,
                systems::sync_mouse_position,
                systems::follow_player_with_camera,
                systems::on_no_players_show_game_over,
                systems::on_click_and_no_player_reset,
                systems::spin_spin_me,
                systems::update_sprite_animation_for_always_animate,
                systems::update_sprite_index,
            )
                .run_if(in_state(AppState::InGame)),
        )
        // Powerup computation systems - run in FixedUpdate
        .add_systems(
            FixedUpdate,
            (
                (
                    systems::powerup_defaulter::<Speed>,
                    systems::powerup_mather::<Speed>,
                )
                    .chain(),
                (
                    systems::powerup_defaulter::<Health>,
                    systems::powerup_mather::<Health>,
                )
                    .chain(),
                (
                    systems::powerup_defaulter::<RateOfFire>,
                    systems::powerup_mather::<RateOfFire>,
                )
                    .chain(),
                (
                    systems::powerup_defaulter::<TimeToLive>,
                    systems::powerup_mather::<TimeToLive>,
                )
                    .chain(),
                (
                    systems::powerup_defaulter::<Poison>,
                    systems::powerup_mather::<Poison>,
                )
                    .chain(),
                (
                    systems::powerup_defaulter::<AmmoCount>,
                    systems::powerup_mather::<AmmoCount>,
                )
                    .chain(),
            )
                .in_set(ComputePowerupsSet)
                .run_if(in_state(AppState::InGame)),
        )
        // Game loop systems - run in FixedUpdate after powerups
        .add_systems(
            FixedUpdate,
            (
                systems::count_ticks,
                systems::shoot_gun,
                systems::move_bullet,
                systems::spawn_mechs,
                systems::spawn_mobs,
                systems::chasers_follow_other_teams,
                systems::player_aimables_aim_at_cursor,
                systems::chaser_aimables_aim_at_other_teams,
                systems::on_scroll_wheel_switch_ammo,
                systems::on_f_key_switch_ammo,
                systems::on_left_click_shoot,
                systems::mob_shoot,
                systems::move_player,
            )
                .in_set(GameLoopSet)
                .after(ComputePowerupsSet)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                systems::move_thing,
                systems::on_move_event_update_sprite_animation,
                systems::on_move_event_update_3d_rotation,
                systems::on_move_event_advance_3d_walking_animation,
                systems::calculate_rate_of_fire,
                systems::rotate_thing,
                systems::team_powerup_assigns_team,
                systems::on_chest_hit_pickup,
                systems::spawn_powerups,
                systems::attach_time_to_live,
                systems::time_to_live,
                systems::on_0_health_kill,
                systems::poison,
                systems::attach_poison,
            )
                .in_set(GameLoopSet)
                .after(ComputePowerupsSet)
                .run_if(in_state(AppState::InGame)),
        )
        // Cleanup systems - run in FixedUpdate after game loop
        .add_systems(
            FixedUpdate,
            systems::consume_despawn_entity_events
                .in_set(GameLoopCleanupSet)
                .after(GameLoopSet)
                .run_if(in_state(AppState::InGame)),
        )
        .configure_sets(FixedUpdate, (ComputePowerupsSet, GameLoopSet, GameLoopCleanupSet).chain())
        .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP as f64))
        .run();
}
