use bevy::prelude::*;

use crate::{events::MoveEvent, util::vector_angle};

pub fn on_move_event_change_sprite_index(
    mut move_events: EventReader<MoveEvent>,
    mut sprites: Query<&mut TextureAtlasSprite>,
){
    move_events.iter().for_each(|event| {
        match sprites.get_mut(event.who).ok() {
            None => (),
            Some(mut sprite) => {
                sprite.index = index_for_direction(event.direction);
            }
        }
    })
}

fn index_for_direction(direction: Vec3) -> usize {
    let angle = vector_angle(direction).to_degrees().round() as usize;

    (angle / 45) % 8 // 360 / 8 
}