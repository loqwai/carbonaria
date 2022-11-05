use bevy::{prelude::Entity, utils::HashSet};
use heron::Collisions;

use crate::components::{SpeedUp, Powerup};

use bevy::prelude::*;

pub fn detect_powerup(
    mut commands: Commands,
    q_things: Query<Entity, Without<Powerup>>,
    q_powerups : Query<(&Collisions, Entity)>,
) {
    let things: HashSet<Entity> = q_things.iter().collect();
    let collisions: HashSet<Entity> = q_powerups.iter().flat_map(|(cs, _)| cs.entities()).collect();

    things.intersection(&collisions).for_each(|thing| {
        commands.entity(*thing).insert(SpeedUp);

        q_powerups.for_each(|(_, powerup)| {
            commands.entity(powerup).despawn();
        })
    });
}