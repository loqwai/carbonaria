use bevy::prelude::*;

use crate::{
    bundles::{CompassBundle, HealthBundle, LaserGunBundle, PlayerBundle},
    components::{RateOfFire, Speed, Health, Math},
    resources::Config,
};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = commands
        .spawn(PlayerBundle::new(
            &asset_server,
            &mut texture_atlases,
            config.scale,
        ))
        .id();
    let compass = commands.spawn(CompassBundle::new(&asset_server)).id();
    let health_ui = commands.spawn(HealthBundle::new(&asset_server)).id();
    let laser_gun = commands.spawn(LaserGunBundle::new(60)).id();
    let rate_of_fire_powerup = commands.spawn(Math::add(RateOfFire(1.0))).id();
    let speed_powerup = commands.spawn(Math::add(Speed(5.0))).id();
    let health_powerup = commands
        .spawn(Math::add(Health(75)))
        .id();

    commands.entity(player).push_children(&[
        health_ui,
        compass,
        laser_gun,
        rate_of_fire_powerup,
        health_powerup,
        speed_powerup,
    ]);
}
