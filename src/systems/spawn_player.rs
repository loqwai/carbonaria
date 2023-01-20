use bevy::prelude::*;

use crate::bundles::{HealthBundle, PlayerBundle, CompassBundle, LaserGunBundle };

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player = commands.spawn(PlayerBundle::new(&asset_server)).id();

    let laser_gun = commands
    .spawn(LaserGunBundle::new(&asset_server))
    .id();
    let compass = commands
        .spawn(CompassBundle::new(&asset_server))
        .id();
    let health_ui = commands.spawn(HealthBundle::new(&asset_server)).id();

    commands.entity(player).push_children(&[laser_gun, health_ui, compass]);
}
