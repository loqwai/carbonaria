use bevy::prelude::Entity;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    components::{Chest, Math, Pocket, Poison},
    events::DespawnEvent,
};

use bevy::prelude::*;

pub fn on_chest_hit_pickup(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut despawn_events: EventWriter<DespawnEvent>,
    q_pockets: Query<Entity, With<Pocket>>,
    q_chests: Query<&Chest>,
    poisons: Query<&Math<Poison>>,
) {
    collision_events
        .iter()
        .filter_map(|collision| match collision {
            CollisionEvent::Started(a, b, _) => {
                if q_pockets.contains(*a) && q_chests.contains(*b) {
                    return Some((a, b));
                }

                if q_pockets.contains(*b) && q_chests.contains(*a) {
                    return Some((b, a));
                }

                return None;
            }
            CollisionEvent::Stopped(_, _, _) => None,
        })
        .for_each(|(&pocket_entity, &chest_entity)| {
            let chest = q_chests.get(chest_entity).unwrap();

            chest.contents.iter().for_each(|&entity| {
                if let Ok(poison) = poisons.get(entity) {
                    println!("{:?} just picked up Poison({:?})!", pocket_entity, poison);
                }
            });

            commands
                .entity(pocket_entity)
                .push_children(&chest.contents);
            despawn_events.send(DespawnEvent {
                entity: chest_entity,
            });
        });
}
