use bevy::prelude::*;
use heron::CollisionEvent;

use crate::components::{Mob, Player, Points, Stick};

pub fn on_stick_hit_increment_points(
    mut q_points: Query<&mut Points, With<Player>>,
    q_sticks: Query<&AnimationPlayer, With<Stick>>,
    q_mobs: Query<&Mob>,
    mut events: EventReader<CollisionEvent>,
) {
    for _ in events
        .iter()
        .filter_map(|e| match e {
            CollisionEvent::Started(t1, t2) => Some(vec![t1, t2]),
            CollisionEvent::Stopped(_, _) => None,
        })
        .filter(|items| {
            items
                .iter()
                .any(|t| q_mobs.get(t.rigid_body_entity()).is_ok())
        })
        .filter_map(|items| {
            items
                .iter()
                .find_map(|t| q_sticks.get(t.rigid_body_entity()).ok())
        })
        .filter(|animation_player| !animation_player.is_paused())
    {
        for mut points in q_points.iter_mut() {
            points.0 += 1;
            println!("Points: {}", points.0);
        }
    }
}
