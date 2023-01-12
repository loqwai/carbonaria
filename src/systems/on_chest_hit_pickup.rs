use bevy::{prelude::Entity};
use heron::Collisions;

use crate::components::{Chest, Pocket};

use bevy::prelude::*;

pub fn on_chest_hit_pickup(
    mut commands: Commands,
    q_pockets: Query<Entity, With<Pocket>>,
    q_powerups : Query<(Entity, &Chest, &Collisions)>,
) {
    for (chest_entity, chest, collisions) in q_powerups.iter() {
        for pocket_entity in collisions.entities() {
            if q_pockets.get(pocket_entity).is_ok() {
                match chest.contents {
                    None => break,
                    Some(contents) => {
                        commands.entity(pocket_entity).push_children(&[contents]);
                        commands.entity(chest_entity).despawn_recursive();
                    }
                }
                break;
            }
        }
    }
}
