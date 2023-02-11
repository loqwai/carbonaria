use bevy::prelude::*;

use crate::{bundles::{RageQuitBulletBundle}, components::{LaserGun, TimeToLive, Chest, RateOfFire, Math, Speed}, resources::Config};

pub fn shoot_rage_quit_gun(
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
        let ttl_powerup = commands.spawn(TimeToLive(100)).insert(Math::add(RateOfFire(1))).id();
        // TODO: replace magic numbers
        let bullet = commands.spawn(RageQuitBulletBundle::new(
            &asset_server,
            &mut texture_atlases,
            &transform.mul_transform(Transform::from_translation(Vec3::new(250.0 * config.scale, 1.0, 1.0))),
            config.scale,
        ))
        .insert(Chest {
            contents: ttl_powerup,
        })
        .id();

        let bullet_time_to_live = commands.spawn(TimeToLive(200)).id();
        let bullet_speed = commands.spawn(Math::add(Speed(10.0))).id();
        commands.entity(bullet).push_children(&[bullet_time_to_live, bullet_speed]);
    })
}
