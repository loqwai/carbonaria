use crate::components::{Tick, Speed};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{bundles::ChestBundle};
use bevy::prelude::*;

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

    let speedup_entity = commands.spawn_empty().insert(Speed::fast()).id();
    let position = random_position(&config, &mut rng);
    commands.spawn(ChestBundle::new(
        &asset_server,
        position,
        speedup_entity,
    ));

}
