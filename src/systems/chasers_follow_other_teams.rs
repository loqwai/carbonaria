use bevy::prelude::*;

use crate::{
    components::{Chases, Team}, events::MoveEvent, util::look_at_target,
};

pub fn chasers_follow_other_teams(
    chasers: Query<(Entity, &Team, &Transform), With<Chases>>,
    targets: Query<(&Team, &Transform)>,
    mut move_events: EventWriter<MoveEvent>,
) {
    chasers.for_each(|(chaser_entity, chaser_team, chaser_transform)| {
        match targets.iter().find(|(team, _)| team != &chaser_team) {
            None => return,
            Some((_, target_transform)) => {
                let (rotation,direction) = look_at_target(chaser_transform.translation, target_transform.translation);
                move_events.send(MoveEvent{
                    who: chaser_entity,
                    velocity: direction,
                    rotation,
                });
            }
        }
    });
}
