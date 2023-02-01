use crate::components::{Tick, Speed, Team, Health, RateOfFire};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{bundles::ChestBundle};
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_powerups(
    mut commands: Commands,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
) {
    let ticks = ticker.0;
    if ticks % config.powerup_spawn_interval != 0 {
        return;
    }
    let (powerup, sprite) = match rng.gen_range(0..5) {
        0 => (commands.spawn(Speed(1.1)).id(), "fast"),
        1 => (commands.spawn(Speed(0.9)).id(), "slow"),
        2 => (commands.spawn(Team(0)).id(), "team"),
        3 => (commands.spawn(Health(1)).id(), "health"),
        4 => (commands.spawn(RateOfFire(2)).id(), "zapper-field"),
        n => panic!("Generated a number not between 0 & 4: {}", n)
    };

    let position = random_position(&config, &mut rng);
    commands.spawn(ChestBundle::new(
        &asset_server,
        position,
        config.scale,
        sprite,
        powerup,
    ));
}

