use bevy::prelude::*;

use crate::resources::MechWalkingAnimation;

pub fn on_move_event_advance_3d_walking_animation(
    mut animation_player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    animation: Res<MechWalkingAnimation>,
) {
    for mut animation_player in animation_player.iter_mut() {
        animation_player
            .play(animation.0.clone())
            .set_speed(0.75)
            .repeat();
    }
}
