use bevy::prelude::*;

use crate::{components::SwingStickAnimation, events::SwingStickEvent};

pub fn begin_swing_stick_animation(
    mut reader: EventReader<SwingStickEvent>,
    mut query: Query<(&SwingStickAnimation, &mut AnimationPlayer)>,
) {
    for event in reader.iter() {
        match query.get_mut(event.stick) {
            Ok((animation, mut player)) => {
                player.play(animation.handle.clone());
            }
            Err(_) => (),
        };
    }
}

pub fn maybe_end_swing_stick_animation(
    mut query: Query<(&mut AnimationPlayer, &mut SwingStickAnimation)>,
) {
    query.for_each_mut(|(mut player, animation)| {
        if player.elapsed() < animation.duration {
            return;
        }

        player.play(animation.reset_handle.clone());
    });
}
