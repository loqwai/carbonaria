use crate::{
    components::{LaserGun, RateOfFire},
    events::DespawnEvent,
};
use bevy::prelude::*;

pub fn move_rate_of_fire_powerup_to_gun(
    rate_of_fires: Query<(Entity, &RateOfFire, &Parent)>,
    childrens: Query<&Children>,
    mut guns: Query<&mut LaserGun>,
    mut despawn_events: EventWriter<DespawnEvent>,
) {
    rate_of_fires.for_each(|(entity, rate_of_fire, parent)| {
        for child in childrens.get(parent.get()).unwrap() {
            match guns.get_mut(*child).ok() {
                None => (),
                Some(mut gun) => {
                    gun.cooldown_max /= rate_of_fire.0;
                    gun.cooldown = gun.cooldown.min(gun.cooldown_max);
                }
            }
        }
        despawn_events.send(DespawnEvent { entity });
    });
}
