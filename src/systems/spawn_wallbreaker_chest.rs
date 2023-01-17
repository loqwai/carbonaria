use bevy::prelude::*;

use crate::{bundles::ChestBundle, components::Wallbreaker};

pub fn spawn_wallbreaker_chest(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wallbreaker_entity = commands.spawn_empty().insert(Wallbreaker).id();
    commands.spawn(ChestBundle::new(
        &asset_server,
        Vec3::new(128.0, 0.0, 0.0),
        wallbreaker_entity,
    ));
}
