use bevy::{prelude::Entity};
use heron::Collisions;

use crate::components::{Chest, Pocket};

use bevy::prelude::*;

pub fn on_chest_hit_pickup(
    mut commands: Commands,
    q_pockets: Query<Entity, With<Pocket>>,
    q_powerups : Query<(Entity, &mut Chest, &Collisions)>,
) {
    for (chest_entity, chest, collisions) in q_powerups.iter() {
        for pocket_entity in collisions.entities() {
            if q_pockets.get(pocket_entity).is_ok() {
                commands.entity(pocket_entity).push_children(&[chest.contents.unwrap()]); //This could be null and blow things up.
                commands.entity(chest_entity).despawn_recursive();
                break;
            }
        }
    }
}
