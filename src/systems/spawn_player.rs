use bevy::prelude::*;

use crate::{bundles::{CompassBundle, HealthBundle, LaserGunBundle, PlayerBundle}, resources::Config};

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = commands.spawn(PlayerBundle::new(&asset_server, &mut texture_atlases, config.scale)).id();
    let laser_gun = commands.spawn(LaserGunBundle::new(&asset_server)).id();
    let compass = commands.spawn(CompassBundle::new(&asset_server)).id();
    let health_ui = commands.spawn(HealthBundle::new(&asset_server)).id();

    commands
        .entity(player)
        .push_children(&[laser_gun, health_ui, compass]);
}
