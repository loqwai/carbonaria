use bevy::prelude::*;

use crate::bundles::{PlayerBundle, StickBundle};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = commands.spawn_bundle(PlayerBundle::new(&asset_server)).id();
    let stick = commands.spawn_bundle(StickBundle::new(&asset_server)).id();

    commands.entity(player).push_children(&[stick]);
}
