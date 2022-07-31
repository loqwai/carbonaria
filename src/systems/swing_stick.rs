use std::f32::consts::PI;

use bevy::prelude::*;

use crate::components::SwingStickAnimation;

pub fn swing_stick(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationPlayer,
        &mut SwingStickAnimation,
        &mut Transform,
        &Name,
    )>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    query.for_each_mut(
        |(stick, mut player, mut swing_stick_animation, mut transform, name)| {
            let animation = swing_stick_animation.handle.get_or_insert_with(|| {
                let handle = animations.add(build_animation(name.clone()));
                player.play(handle.clone()).stop_repeating();
                handle
            });

            if animation_done_playing(&player) {
                animations.remove(animation.clone());
                commands.entity(stick).remove::<SwingStickAnimation>();
                transform.rotation = Quat::from_axis_angle(Vec3::Z, -PI);

                return;
            }
        },
    );
}

fn animation_done_playing(player: &AnimationPlayer) -> bool {
    player.elapsed() > 0.2
}

fn build_animation(stick: Name) -> AnimationClip {
    let mut animation: AnimationClip = Default::default();

    animation.add_curve_to_path(
        EntityPath {
            parts: vec![stick.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.2],
            keyframes: Keyframes::Rotation(vec![
                Quat::from_axis_angle(Vec3::Z, -PI),
                Quat::from_axis_angle(Vec3::Z, 0.0),
            ]),
        },
    );

    animation
}
