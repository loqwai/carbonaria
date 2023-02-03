use crate::{components::Health, events::DespawnEvent};
use bevy::prelude::*;
pub fn health_powerup_add_health(
    powerups: Query<(Entity, &Parent, &Health)>,
    mut parent_healths: Query<&mut Health, Without<Parent>>,
    mut despawn_events: EventWriter<DespawnEvent>,
) {
    powerups.for_each(|(entity, parent, powerup_health)| {
        match parent_healths.get_mut(parent.get()) {
            Ok(mut parent_health) => {
                parent_health.0 += powerup_health.0;
                
                despawn_events.send(DespawnEvent { entity });
            }
            Err(_) => {
                panic!("Somehow we found a powerup that belongs to a parent that doesn't exist")
            }
        }
    });
}
