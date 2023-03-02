use crate::{
    components::{LaserGun, Mob},
    events::ShootEvent,
};
use bevy::prelude::*;

pub fn mob_shoot(
    mobs: Query<&Children, With<Mob>>,
    mut shoot_events: EventWriter<ShootEvent>,
    guns: Query<Entity, With<LaserGun>>,
) {
    for mob in mobs.iter() {
        for &child in mob.iter() {
            let Ok(gun) = guns.get(child) else { continue; };
            shoot_events.send(ShootEvent { gun });
        }
    }
}
