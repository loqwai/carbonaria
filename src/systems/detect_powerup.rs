use bevy::{prelude::Entity, utils::HashSet};
use heron::Collisions;

use crate::components::{SpeedUp, Chest};

use bevy::prelude::*;

pub fn detect_powerup(
    mut commands: Commands,
    q_pockets: Query<Entity, Without<Chest>>,
    q_powerups : Query<(Entity, &mut Chest, &Collisions)>,
) {
    for (chest_entity, chest, collisions) in q_powerups.iter() {
        for pocket_entity in collisions.entities() {
            if q_pockets.get(pocket_entity).is_ok() {
                commands.entity(pocket_entity).push_children(&[chest.contents.unwrap()]);
                commands.entity(chest_entity).despawn_recursive();
                break;
            }
        }
    }
}
