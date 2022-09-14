use bevy::prelude::*;

use crate::bundles::{HealthBundle, PlayerBundle, StickBundle};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    animations: ResMut<Assets<AnimationClip>>,
) {
    let player = commands.spawn_bundle(PlayerBundle::new(&asset_server)).id();

    let stick = commands
        .spawn_bundle(StickBundle::new(&asset_server, animations))
        .id();

    commands.entity(player).push_children(&[stick]);

    commands.spawn_bundle(HealthBundle::new(&asset_server, player));
}
