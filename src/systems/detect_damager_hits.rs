use bevy::prelude::Entity;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    components::{Damage, Health},
    events::DamagerHitEvent,
};

use bevy::prelude::*;

pub fn detect_damager_hits(
    mut collision_events: EventReader<CollisionEvent>,
    mut damager_hit_events: EventWriter<DamagerHitEvent>,
    healthies: Query<Entity, With<Health>>,
    damagers: Query<&Damage>,
) {
    collision_events
        .iter()
        .filter_map(|collision| match collision {
            CollisionEvent::Started(a, b, _) => {
                if healthies.contains(*a) && damagers.contains(*b) {
                    return Some((a, b));
                }

                if healthies.contains(*b) && damagers.contains(*a) {
                    return Some((b, a));
                }

                return None;
            }
            CollisionEvent::Stopped(_, _, _) => None,
        })
        .for_each(|(&target, &damager)| {
            // let damager = damagers.get(damager).unwrap();
            damager_hit_events.send(DamagerHitEvent { damager, target })
        });
}
