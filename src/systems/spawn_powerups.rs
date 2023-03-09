use crate::bundles::ChestBundle;
use crate::components::{Health, Math, RateOfFire, Speed, Team, Tick};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_powerups(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ticks = ticker.0;
    if ticks % config.powerup_spawn_interval != 0 {
        return;
    }
    let (powerup, sprite, tile_size, sprite_sheet_dimensions) = match rng.gen_range(0..5) {
        0 => (
            commands.spawn(Math::add(Speed(1.0))).id(),
            "fast",
            Vec2::new(512.0, 512.0),
            (1, 1),
        ),
        1 => (
            commands.spawn(Math::add(Speed(-1.0))).id(),
            "slow",
            Vec2::new(512.0, 512.0),
            (1, 1),
        ),
        2 => (
            commands.spawn(Team(0)).id(),
            "team",
            Vec2::new(512.0, 512.0),
            (1, 1),
        ),
        3 => (
            commands.spawn(Math::add(Health(1.0))).id(),
            "health",
            Vec2::new(128.0, 128.0),
            (8, 8),
        ),
        4 => (
            commands.spawn(Math::add(RateOfFire(2.0))).id(),
            "rate-of-fire",
            Vec2::new(512.0, 512.0),
            (1, 1),
        ),
        n => panic!("Generated a number not between 0 & 4: {}", n),
    };

    let position = random_position(&config, &mut rng);
    commands.spawn(ChestBundle::new(
        &asset_server,
        &mut texture_atlases,
        &mut meshes,
        &mut materials,
        position,
        config.scale,
        sprite,
        tile_size,
        sprite_sheet_dimensions,
        vec![powerup],
    ));
}
