use bevy::prelude::*;

use crate::{
    bundles::BulletBundle,
    components::{
        ActiveAmmo, AmmoType, Chest, Health, LaserGun, Math, Poison, RateOfFire, Speed, TimeToLive,
    },
    resources::Config,
};

pub fn shoot_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut guns: Query<(&mut LaserGun, &ActiveAmmo, &GlobalTransform)>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    guns.for_each_mut(|(mut gun, active_ammo, transform)| {
        if gun.cooldown > 0 {
            gun.cooldown = gun.cooldown.saturating_sub(gun.cooldown_rate);
            return;
        }

        gun.cooldown = gun.cooldown_max;

        let payload = match active_ammo.0 {
            AmmoType::Normal => commands.spawn(Math::add(Health(-10.0))).id(),
            AmmoType::Poison => commands
                .spawn(Math::add(Poison(0.1)))
                .with_children(|payload| {
                    payload.spawn(Math::add(TimeToLive(60 * 5)));
                })
                .id(),
            AmmoType::RageQuit => commands
                .spawn_empty()
                .insert(Math::add(TimeToLive(100)))
                .insert(Math::multiply(RateOfFire(0.1)))
                .id(),
            AmmoType::Reverser => commands
                .spawn_empty()
                .insert(Math::multiply(Speed(-1.0)))
                .with_children(|parent| {
                    parent.spawn(Math::add(TimeToLive(300)));
                })
                .id(),
        };

        let texture = match active_ammo.0 {
            AmmoType::Normal => "laser",
            AmmoType::Poison => "poison",
            AmmoType::RageQuit => "ragequit",
            AmmoType::Reverser => "reverser",
        };

        // TODO: replace magic numbers
        commands
            .spawn(BulletBundle::new(
                &asset_server,
                &mut texture_atlases,
                &transform.mul_transform(Transform::from_translation(Vec3::new(
                    250.0 * config.scale,
                    1.0,
                    1.0,
                ))),
                texture,
                config.scale,
            ))
            .insert(Chest {
                contents: vec![payload],
            })
            .with_children(|parent| {
                parent.spawn(Math::add(TimeToLive(200)));
                parent.spawn(Math::add(Speed(10.0)));
            });
    })
}
