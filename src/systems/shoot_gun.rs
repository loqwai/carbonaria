use bevy::prelude::*;

use crate::{bundles::LaserGunBulletBundle, components::LaserGun, resources::Config};

pub fn shoot_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut guns: Query<(&mut LaserGun, &GlobalTransform)>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    guns.for_each_mut(|(mut gun, transform)| {
        if gun.cooldown > 0 {
            gun.cooldown -= 1;
            return;
        }

        gun.cooldown = gun.cooldown_max;
        // TODO: replace magic numbers
        commands.spawn(LaserGunBulletBundle::new(
            &asset_server,
            &mut texture_atlases,
            &transform.mul_transform(Transform::from_translation(Vec3::new(225.0 * config.scale, 1.0, 1.0))),
            config.scale,
        ));
    })
}
