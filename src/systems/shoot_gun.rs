use bevy::prelude::*;

use crate::{
    bundles::{BulletBundle, BulletModelBundle},
    components::{
        ActiveAmmo, AmmoCount, AmmoType, Chest, Health, LaserGun, Math, Poison, RateOfFire, Speed,
        TimeToLive,
    },
    events::ShootEvent,
    resources::Config,
};

pub fn shoot_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shoot_events: EventReader<ShootEvent>,
    mut guns: Query<(&mut LaserGun, &Children, &ActiveAmmo, &GlobalTransform)>,
    mut clips: Query<(Entity, &AmmoCount, With<Parent>)>,
    config: Res<Config>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    guns.for_each_mut(|(mut gun, _, _, _)| {
        if gun.cooldown > 0 {
            gun.cooldown = gun.cooldown.saturating_sub(gun.cooldown_rate);
        }
    });

    for (gun, _,_,_) in guns.iter() {
        if gun.cooldown > 0 {
            gun.cooldown = gun.cooldown.saturating_sub(gun.cooldown_rate);
        }
    }

    for event in shoot_events.iter() {
        let Ok((mut gun, children, active_ammo, transform)) = guns.get_mut(event.gun) else { continue; };

        if gun.cooldown > 0 {
            continue;
        }
        for &child in children.iter() {
            let (clip, count, _) = clips.get(child).unwrap();
            commands.entity(clip).remove::<Parent>();
            if count.0 <= 0 {
                continue;
            }
            gun.cooldown = gun.cooldown_max;        gun.cooldown = gun.cooldown_max;
        }

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

        let model_name = "laser";

        // TODO: replace magic numbers
        commands
            .spawn(BulletBundle::new(
                &asset_server,
                &mut texture_atlases,
                transform
                    .mul_transform(Transform::from_translation(Vec3::new(
                        250.0 * config.scale,
                        1.0,
                        1.0,
                    )))
                    .compute_transform(),
                texture,
                config.scale,
            ))
            .insert(Chest {
                contents: vec![payload],
            })
            .with_children(|parent| {
                parent.spawn(BulletModelBundle::new(
                    &asset_server,
                    config.scale,
                    model_name,
                ));
                parent.spawn(Math::add(TimeToLive(200)));
                parent.spawn(Math::add(Speed(10.0)));
            });
    }
}
