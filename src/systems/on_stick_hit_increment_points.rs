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
            CollisionEvent::Started(t1, t2) => Some((t1, t2)),
            CollisionEvent::Stopped(_, _) => None,
        })
        .filter(|(maybe_stick, maybe_mob)| {
            if let Err(_) = q_mobs.get(maybe_mob.rigid_body_entity()) {
                return false;
            }

            match q_sticks.get(maybe_stick.rigid_body_entity()) {
                Err(_) => false,
                Ok(animation_player) => !animation_player.is_paused(),
            }
        })
    {
        for mut points in q_points.iter_mut() {
            points.0 += 1;
            println!("Points: {}", points.0);
        }
    }
}
