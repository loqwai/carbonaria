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
    match rng.gen_range(0..4) {
        0 => {
            let powerup = Speed::fast();
            let powerup_entity = commands.spawn(powerup).id();
            let sprite = "fast";
            let position = random_position(&config, &mut rng);
            commands.spawn(ChestBundle::new(
                &asset_server,
                position,
                &sprite,
                powerup_entity,
            ));
        }
        1 => {
            let powerup = Speed::slow();
            let powerup_entity = commands.spawn(powerup).id();
            let sprite = "slow";
            let position = random_position(&config, &mut rng);
            commands.spawn(ChestBundle::new(
                &asset_server,
                position,
                &sprite,
                powerup_entity,
            ));
        }
        2 => {
            let powerup = Team(1);
            let powerup_entity = commands.spawn(powerup).id();
            let sprite = "team";
            let position = random_position(&config, &mut rng);
            commands.spawn(ChestBundle::new(
                &asset_server,
                position,
                &sprite,
                powerup_entity,
            ));
        }
        3 => {
            let powerup = Health(1);
            let powerup_entity = commands.spawn(powerup).id();
            let sprite = "health";
            let position = random_position(&config, &mut rng);
            commands.spawn(ChestBundle::new(
                &asset_server,
                position,
                &sprite,
                powerup_entity,
            ));
        },
        _ => (),
    }
}

