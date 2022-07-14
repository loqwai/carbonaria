use bevy::{prelude::*, sprite::SpriteBundle};

use crate::resources::Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut player: ResMut<Player>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let entity = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        })
        .id();

    player.entity = Some(entity);
}
