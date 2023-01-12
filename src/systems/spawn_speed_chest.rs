use bevy::prelude::*;

use crate::{bundles::ChestBundle, components::Speed};

pub fn spawn_speed_chest(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_speed_up(&mut commands, &asset_server);
    spawn_speed_down(&mut commands, &asset_server);
}

fn spawn_speed_up(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let speedup_entity = commands.spawn().insert(Speed::fast()).id();
    commands.spawn_bundle(ChestBundle::new(
        &asset_server,
        Vec3::new(128.0, 128.0, 0.0),
        speedup_entity,
    ));
}

fn spawn_speed_down(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let speedup_entity = commands.spawn().insert(Speed::slow()).id();
    commands.spawn_bundle(ChestBundle::new(
        &asset_server,
        Vec3::new(0.0, 128.0, 0.0),
        speedup_entity,
    ));
}
