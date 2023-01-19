use bevy::prelude::*;

use crate::{bundles::LaserGunBulletBundle, components::LaserGun};

pub fn shoot_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    guns: Query<&GlobalTransform, With<LaserGun>>,
) {
    guns.for_each(|gun| {
        commands.spawn(LaserGunBulletBundle::new(&asset_server, &gun));
    })
}
