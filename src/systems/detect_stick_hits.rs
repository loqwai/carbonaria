use bevy::prelude::*;
use heron::CollisionEvent;

use crate::{components::Stick, events::StickHitEvent};

pub fn detect_stick_hits(
    q_sticks: Query<&Stick>,
    mut collision_events: EventReader<CollisionEvent>,
    mut stick_hit_events: EventWriter<StickHitEvent>,
) {
    for (stick, target) in collision_events
        .iter()
        .filter_map(|e| match e {
            CollisionEvent::Started(t1, t2) => {
                Some(vec![t1.rigid_body_entity(), t2.rigid_body_entity()])
            }
            CollisionEvent::Stopped(_, _) => None,
        })
        .filter_map(|items| {
            let stick = items.iter().find(|&t| q_sticks.get(*t).is_ok());
            let target = items.iter().find(|&t| Some(t) != stick);

            match (stick, target) {
                (Some(stick), Some(target)) => Some((*stick, *target)),
                _ => None,
            }
        })
    {
        stick_hit_events.send(StickHitEvent { stick, target })
    }
}
