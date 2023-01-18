use bevy::prelude::*;
use crate::{bundles::ChestBundle, components::Team};

pub fn spawn_powerups(mut commands: Commands, asset_server: Res<AssetServer>) {
    let team_powerup = commands.spawn().insert(Team(0)).id();
    commands.spawn_bundle(ChestBundle::new(
        &asset_server,
        Vec3::new(-128.0, 0.0, 0.0),
        team_powerup,
    ));
}
