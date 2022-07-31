use std::f32::consts::PI;

use bevy::prelude::*;

use crate::components::SwingStickAnimation;

pub fn begin_swing_stick_animation(
    mut query: Query<(&SwingStickAnimation, &mut AnimationPlayer), Added<SwingStickAnimation>>,
) {
    query.for_each_mut(|(animation, mut player)| {
        player.play(animation.handle.clone());
    });
}

pub fn maybe_end_swing_stick_animation(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationPlayer,
        &mut SwingStickAnimation,
        &mut Transform,
    )>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    query.for_each_mut(|(stick, player, animation, mut transform)| {
        if player.elapsed() < animation.duration {
            return;
        }

        animations.remove(animation.handle.clone());
        commands.entity(stick).remove::<SwingStickAnimation>();
        transform.rotation = Quat::from_axis_angle(Vec3::Z, -PI);
    });
}
