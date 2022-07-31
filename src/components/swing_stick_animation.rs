use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct SwingStickAnimation {
    pub handle: Handle<AnimationClip>,
    pub duration: f32,
}

impl SwingStickAnimation {
    pub fn new(animations: &mut ResMut<Assets<AnimationClip>>, name: Name) -> SwingStickAnimation {
        SwingStickAnimation {
            handle: animations.add(build_animation(name)),
            duration: 0.2,
        }
    }
}

impl Default for SwingStickAnimation {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            duration: 0.0,
        }
    }
}

fn build_animation(name: Name) -> AnimationClip {
    let mut animation: AnimationClip = Default::default();

    animation.add_curve_to_path(
        EntityPath {
            parts: vec![name.clone()],
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
