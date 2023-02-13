use bevy::prelude::*;

use crate::{bundles::BulletBundle, components::{LaserGun, TimeToLive, Math, Speed, Chest, Poison}, resources::Config};

pub fn shoot_poison_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut guns: Query<(&mut LaserGun, &GlobalTransform)>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    guns.for_each_mut(|(mut gun, transform)| {
        if gun.cooldown > 0 {
            gun.cooldown = gun.cooldown.saturating_sub(gun.cooldown_rate);
            return;
        }

        gun.cooldown = gun.cooldown_max;
        let payload = commands.spawn(Math::add(Poison(1))).id();
        // TODO: replace magic numbers
        commands.spawn(BulletBundle::new(
            &asset_server,
            &mut texture_atlases,
            &transform.mul_transform(Transform::from_translation(Vec3::new(250.0 * config.scale, 1.0, 1.0))),
            "laser",
            config.scale,
        ))
        .insert(Chest {
            contents: vec![payload],
        }).with_children(|parent| {
            parent.spawn(Math::add(TimeToLive(200)));
            parent.spawn(Math::add(Speed(10.0)));
        });
    })
}
