use crate::components::{Tick, Speed, Team, Health};
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
    let (powerup, sprite) = match rng.gen_range(0..4) {
        0 => (commands.spawn(Speed::fast()).id(), "fast"),
        1 => (commands.spawn(Speed::slow()).id(), "team"),
        2 => (commands.spawn(Team(1)).id(), "team"),
        3 => (commands.spawn(Health(1)).id(), "health"),
        n => panic!("Generated a number not between 0 & 4: {}", n)
    };

    let position = random_position(&config, &mut rng);
    commands.spawn(ChestBundle::new(
        &asset_server,
        position,
        sprite,
        powerup,
    ));
}

