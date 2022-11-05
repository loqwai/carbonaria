use bevy::prelude::*;

use crate::bundles::{HealthBundle, PlayerBundle, StickBundle, CompassBundle };

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    animations: ResMut<Assets<AnimationClip>>,
) {
    let player = commands.spawn_bundle(PlayerBundle::new(&asset_server)).id();

    let stick = commands
        .spawn_bundle(StickBundle::new(&asset_server, animations))
        .id();
    let compass = commands
        .spawn_bundle(CompassBundle::new(&asset_server))
        .id();
    let health_ui = commands.spawn_bundle(HealthBundle::new(&asset_server)).id();

    commands.entity(player).push_children(&[stick, health_ui, compass]);
}