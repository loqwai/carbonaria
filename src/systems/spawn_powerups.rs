use crate::bundles::{ChestBundle, ChestFallbackModelBundle, ChestModelBundle};
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
    let (powerup, sprite, tile_size, sprite_sheet_dimensions, num_frames, model) =
        match rng.gen_range(0..5) {
            0 => (
                commands.spawn(Math::add(Speed(1.0))).id(),
                "fast",
                Vec2::new(128.0, 128.0),
                (8, 8),
                63,
                Some("fast"),
            ),
            1 => (
                commands.spawn(Math::add(Speed(-1.0))).id(),
                "slow",
                Vec2::new(512.0, 512.0),
                (1, 1),
                1,
                None,
            ),
            2 => (
                commands.spawn(Team(0)).id(),
                "team",
                Vec2::new(512.0, 512.0),
                (1, 1),
                1,
                None,
            ),
            3 => (
                commands.spawn(Math::add(Health(1.0))).id(),
                "health",
                Vec2::new(128.0, 128.0),
                (8, 8),
                63,
                Some("health"),
            ),
            4 => (
                commands.spawn(Math::add(RateOfFire(2.0))).id(),
                "rate-of-fire",
                Vec2::new(512.0, 512.0),
                (1, 1),
                1,
                None,
            ),
            n => panic!("Generated a number not between 0 & 4: {}", n),
        };

    let position = random_position(&config, &mut rng);
    commands
        .spawn(ChestBundle::new(
            &asset_server,
            &mut texture_atlases,
            position,
            config.scale,
            sprite,
            tile_size,
            sprite_sheet_dimensions,
            num_frames,
            vec![powerup],
        ))
        .with_children(|parent| match model {
            Some(model) => {
                parent.spawn(ChestModelBundle::new(&asset_server, config.scale, model));
            }
            None => {
                parent.spawn(ChestFallbackModelBundle::new(
                    &mut meshes,
                    &mut materials,
                    config.scale,
                ));
            }
        });
}
