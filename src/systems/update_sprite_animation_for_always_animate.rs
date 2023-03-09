use crate::components::{AlwaysAnimate, SpriteAnimation};
use bevy::prelude::*;

pub fn update_sprite_animation_for_always_animate(
    mut sprite_animations: Query<&mut SpriteAnimation, With<AlwaysAnimate>>,
) {
    sprite_animations.iter_mut().for_each(|mut animation| {
        animation.current_frame = clamp(
            animation.current_frame + animation.frames_to_advance_per_tick,
            animation.num_frames_per_angle,
        );
    });
}

fn clamp(value: f32, max: usize) -> f32 {
    if value < max as f32 {
        return value;
    }

    return clamp(value - max as f32, max);
}
