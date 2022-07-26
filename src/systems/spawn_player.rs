use bevy::prelude::*;

use crate::bundles::PlayerBundle;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PlayerBundle::new(asset_server));
}
