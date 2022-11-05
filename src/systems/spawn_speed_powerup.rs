use bevy::prelude::*;

use crate::bundles::SpeedPowerupBundle;

pub fn spawn_speed_powerup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpeedPowerupBundle::new(&asset_server, Vec3::new(128.0, 128.0, 0.0)));
}