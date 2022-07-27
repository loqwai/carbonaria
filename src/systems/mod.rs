mod move_player;
mod spawn_camera;
mod spawn_mobs;
mod spawn_room;
mod update_carried_item_locations;

use bevy::prelude::*;

use crate::bundles::{PlayerBundle, StickBundle};

pub use move_player::move_player;
pub use spawn_camera::spawn_camera;
pub use spawn_mobs::spawn_mobs;
pub use spawn_room::spawn_room;
pub use update_carried_item_locations::update_carried_item_locations;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PlayerBundle::new(asset_server));
}

pub fn spawn_stick(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(StickBundle::new(asset_server, Vec3::new(128.0, 128.0, 0.0)));
}
