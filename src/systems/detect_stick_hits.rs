use bevy::prelude::*;
use heron::CollisionEvent;

use crate::{
    components::{Mob, Stick},
    events::StickHitEvent,
};

pub fn detect_stick_hits(
    q_sticks: Query<&Stick>,
    q_mobs: Query<&Mob>,
    mut collision_events: EventReader<CollisionEvent>,
    mut stick_hit_events: EventWriter<StickHitEvent>,
) {
    for (stick, mob) in collision_events
        .iter()
        .filter_map(|e| match e {
            CollisionEvent::Started(t1, t2) => {
                Some(vec![t1.rigid_body_entity(), t2.rigid_body_entity()])
            }
            CollisionEvent::Stopped(_, _) => None,
        })
        .filter_map(|items| {
            let stick = items.iter().find(|&t| q_sticks.get(*t).is_ok());
            let mob = items.iter().find(|&t| q_mobs.get(*t).is_ok());

            match (stick, mob) {
                (Some(stick), Some(mob)) => Some((*stick, *mob)),
                _ => None,
            }
        })
    {
        stick_hit_events.send(StickHitEvent { stick, mob })
    }
}
