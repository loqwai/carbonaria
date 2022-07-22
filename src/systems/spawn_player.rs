use bevy::{prelude::*, sprite::SpriteBundle};

use crate::{bundles::PlayerBundle, resources::PlayerResource};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<PlayerResource>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let entity = commands
        .spawn_bundle(PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    player.entity = Some(entity);
}
