use bevy::prelude::*;

use crate::{components::SpriteAnimation, events::MoveEvent, util::index_for_direction};

pub fn on_move_event_update_sprite_animation(
    mut move_events: EventReader<MoveEvent>,
    mut sprite_animations: Query<&mut SpriteAnimation>,
) {
    move_events.iter().for_each(|event| {
        set_sprite_index(&mut sprite_animations, event);
    })
}

fn set_sprite_index(
    sprite_animations: &mut Query<&mut SpriteAnimation>,
    event: &MoveEvent,
) -> Option<()> {
    let mut animation = sprite_animations.get_mut(event.who).ok()?;
    animation.current_angle = index_for_direction(event.direction, animation.num_angles);
    animation.current_frame = (animation.current_frame + 1) % animation.num_frames_per_angle;

    return None;
}
