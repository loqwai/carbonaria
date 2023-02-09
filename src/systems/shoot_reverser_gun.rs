use bevy::prelude::*;

use crate::{
    bundles::ReverserBulletBundle,
    components::{AddPowerup, Chest, LaserGun, MultiplyPowerup, Speed, TimeToLive},
    resources::Config,
};

pub fn shoot_reverser_gun(
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
        let payload = commands.spawn_empty()
            .insert(MultiplyPowerup(Speed(-1.0)))
            .with_children(|parent| {
                parent.spawn(TimeToLive(300));
            }).id();

        // TODO: replace magic numbers
        let bullet = commands
            .spawn(ReverserBulletBundle::new(
                &asset_server,
                &mut texture_atlases,
                &transform.mul_transform(Transform::from_translation(Vec3::new(
                    250.0 * config.scale,
                    1.0,
                    1.0,
                ))),
                config.scale,
            ))
            .insert(Chest { contents: payload })
            .id();

        commands.entity(bullet).with_children(|parent| {
            parent.spawn(TimeToLive(200));
            parent.spawn(AddPowerup(Speed(10.0)));
        });
    })
}
