use bevy::prelude::*;

use crate::{
    components::{Points, Stick},
    events::StickHitEvent,
};

pub fn on_stick_hit_increment_points(
    mut events: EventReader<StickHitEvent>,
    q_parent: Query<(&Parent, &AnimationPlayer), With<Stick>>,
    mut q_points: Query<&mut Points>,
) {
    for event in events.iter() {
        match q_parent.get(event.stick) {
            Err(_) => continue,
            Ok((parent, animation_player)) => {
                if animation_player.is_paused() {
                    return;
                }

                match q_points.get_mut(**parent) {
                    Err(_) => continue,
                    Ok(mut points) => {
                        points.0 += 1;
                        println!("points: {}", points.0);
                    }
                }
            }
        };
    }
}
