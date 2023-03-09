use bevy::prelude::*;

use crate::{
    bundles::{CompassBundle, HealthBundle, LaserGunBundle, PlayerBundle, PlayerModelBundle},
    components::{AmmoCount, Health, Math, RateOfFire, Speed},
    resources::Config,
};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(PlayerBundle::new(
            &asset_server,
            &mut texture_atlases,
            config.scale,
        ))
        .with_children(|parent| {
            parent.spawn(PlayerModelBundle::new(&asset_server, config.scale));
            parent.spawn(CompassBundle::new(&asset_server));
            parent.spawn(HealthBundle::new(&asset_server));
            parent.spawn(LaserGunBundle::new(60)).with_children(|gun| {
                gun.spawn(Math::add(AmmoCount(5)));
            });
            parent.spawn(Math::add(AmmoCount(5)));
            parent.spawn(Math::add(RateOfFire(1.0)));
            parent.spawn(Math::add(Speed(7.5)));
            parent.spawn(Math::add(Health(750.0)));
        });
}
