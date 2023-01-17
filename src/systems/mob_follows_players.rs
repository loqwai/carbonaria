use bevy::prelude::*;

use crate::{
    components::{Chases, Team}, events::MoveEvent,
};

pub fn mob_follows_players(
    chasers: Query<(Entity, &Team, &Transform), With<Chases>>,
    targets: Query<(&Team, &Transform)>,
    mut move_events: EventWriter<MoveEvent>,
) {
    chasers.for_each(|(chaser_entity, chaser_team, chaser_transform)| {
        match targets.iter().find(|(team, _)| team != &chaser_team) {
            None => return,
            Some((_, target_transform)) => {
                let diff =  chaser_transform.translation - target_transform.translation;
                let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally

                move_events.send(MoveEvent{
                    who: chaser_entity,
                    velocity: diff.normalize() * -1.0,
                    rotation: Quat::from_axis_angle(Vec3::Z, angle),
                });
            }
        }
    });
}
