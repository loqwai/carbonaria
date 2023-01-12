use bevy::prelude::*;

use crate::{bundles::ChestBundle, components::Speed};

pub fn spawn_speed_chest(mut commands: Commands, asset_server: Res<AssetServer>) {
    let speedup_entity = commands.spawn().insert(Speed::default()).id();
    commands.spawn_bundle(ChestBundle::new(
        &asset_server,
        Vec3::new(128.0, 128.0, 0.0),
        speedup_entity,
    ));
}
