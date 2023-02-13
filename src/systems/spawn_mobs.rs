use bevy::prelude::*;

use crate::components::{Tick, RateOfFire, Math, Speed, Health};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{
    bundles::{LaserGunBundle, MobBundle},
};

pub fn spawn_mobs(
    mut commands: Commands,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ticks = ticker.0;
    if ticks % config.mob_spawn_interval != 0 {
        return;
    }
    let position = random_position(&config, &mut rng);
    commands.spawn(MobBundle::new(&asset_server, &mut texture_atlases, position, config.scale))
    .with_children(|parent| {
        parent.spawn(LaserGunBundle::new(60));
        parent.spawn(Math::add(RateOfFire(1.0)));
        parent.spawn(Math::add(Speed(1.0)));
        parent.spawn(Math::add(Health(10)));
    });
}
