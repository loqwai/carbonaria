use bevy::prelude::*;

use crate::{bundles::LaserGunBulletBundle, components::{LaserGun, TimeToLive, Math, Speed, Health, Chest}, resources::Config};

pub fn shoot_gun(
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

        //TODO: This will not work well when we start having multipliers
        let health_powerdown = commands.spawn(Math{
            add: Some(Health(-10)),
            multiply: None,
        }).id();
        // TODO: replace magic numbers
        let bullet = commands.spawn(LaserGunBulletBundle::new(
            &asset_server,
            &mut texture_atlases,
            &transform.mul_transform(Transform::from_translation(Vec3::new(250.0 * config.scale, 1.0, 1.0))),
            config.scale,
        ))
        .insert(Chest {
            contents: health_powerdown,
        })
        .id();

        let time_to_live = commands.spawn(TimeToLive(200)).id();
        let speed_powerup = commands.spawn(Math{ add: Some(Speed(10.0)), multiply: None }).id();
        let health_powerup = commands.spawn(AddPowerup(Health(1))).id();
        commands.entity(bullet).push_children(&[time_to_live, speed_powerup, health_powerup]);

    })
}
