use bevy::prelude::*;

use crate::bundles::{LaserGunBundle, MechBundle, MechModelBundle};
use crate::components::{Health, Math, RateOfFire, Speed, Tick};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;

pub fn spawn_mechs(
    mut commands: Commands,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ticks = ticker.0;
    if ticks % config.mech_spawn_interval != 0 {
        return;
    }
    let position = random_position(&config, &mut rng);
    commands
        .spawn(MechBundle::new(
            &asset_server,
            &mut texture_atlases,
            position,
            config.scale,
        ))
        .with_children(|parent| {
            parent.spawn(MechModelBundle::new(&asset_server, config.scale));
            parent.spawn(LaserGunBundle::new(60));
            parent.spawn(Math::add(RateOfFire(1.0)));
            parent.spawn(Math::add(Speed(1.0)));
            parent.spawn(Math::add(Health(100.0)));
        });
}
