use bevy::prelude::*;

use crate::{bundles::PlayerBundle, resources::PlayerResource};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<PlayerResource>,
) {
    let entity = commands.spawn_bundle(PlayerBundle::new(asset_server)).id();

    player.entity = Some(entity);
}
