use crate::components::SpriteAnimation;
use bevy::prelude::*;

pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlas, &SpriteAnimation)>) {
    for (mut atlas, animation) in sprites.iter_mut() {
        atlas.index = (animation.current_angle * animation.num_frames_per_angle)
            + animation.current_frame as usize;
    }
}
