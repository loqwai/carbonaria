use crate::components::SpriteAnimation;
use bevy::prelude::*;

pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>) {
    sprites.for_each_mut(|(mut sprite, animation)| {
        sprite.index =
            (animation.current_angle * animation.num_frames_per_angle) + animation.current_frame;
    })
}
