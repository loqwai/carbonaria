use bevy::prelude::*;
use heron::Collisions;

use crate::{components::Wallbreaker, events::StickHitEvent};

pub fn on_stick_hit_wallbreaker(
    mut commands: Commands,
    mut events: EventReader<StickHitEvent>,
    q_equipped_wallbreakers: Query<(Entity, Parent), With<Wallbreaker>>,
) {
    for event in events.iter() {
        for (wallbreaker_entity, parent) in q_equipped_wallbreakers.iter() {
            if parent.0 == event.stick {
                commands.entity(wallbreaker_entity).despawn_recursive();
            }
        }
    }
    for (stick_entity, parent, collisions) in q_powerups.iter() {
        for pocket_entity in collisions.entities() {
            if q_pockets.get(pocket_entity).is_ok() {
                commands
                    .entity(pocket_entity)
                    .push_children(&[chest.contents.unwrap()]); //This could be null and blow things up.
                commands.entity(chest_entity).despawn_recursive();
                break;
            }
        }
    }
}
