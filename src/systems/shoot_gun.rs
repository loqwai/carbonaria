use bevy::prelude::*;

use crate::{bundles::LaserGunBulletBundle, components::LaserGun};

pub fn shoot_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut guns: Query<(&mut LaserGun, &GlobalTransform)>,
) {
    guns.for_each_mut(|(mut gun, transform)| {
        if gun.cooldown > 0.0 {
            gun.cooldown -= 1.0;
            return;
        }

        gun.cooldown = gun.cooldown_max;
        commands.spawn(LaserGunBulletBundle::new(&asset_server, &transform));
    })
}
