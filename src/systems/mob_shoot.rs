use crate::{
    components::{AutoShoot, LaserGun},
    events::ShootEvent,
};
use bevy::prelude::*;

pub fn mob_shoot(
    mobs: Query<(Entity, &Children), With<AutoShoot>>,
    mut shoot_events: EventWriter<ShootEvent>,
    guns: Query<Entity, With<LaserGun>>,
) {
    for (shot_by, mob) in mobs.iter() {
        for &child in mob.iter() {
            let Ok(gun) = guns.get(child) else { continue; };
            shoot_events.send(ShootEvent { gun, shot_by});
        }
    }
}
