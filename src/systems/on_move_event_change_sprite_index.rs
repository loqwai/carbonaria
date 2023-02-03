use bevy::prelude::*;

use crate::{events::MoveEvent, util::vector_angle};

pub fn on_move_event_change_sprite_index(
    mut move_events: EventReader<MoveEvent>,
    mut sprites: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    atlases: Res<Assets<TextureAtlas>>,
){
    move_events.iter().for_each(|event| {
        match sprites.get_mut(event.who).ok() {
            None => (),
            Some((mut sprite, atlas_handle)) => {
                if let Some(atlas) = atlases.get(atlas_handle) {
                    sprite.index = index_for_direction(event.direction, atlas.len());
                }
            }
        }
    })
}

fn index_for_direction(direction: Vec3, len: usize) -> usize {
    let angle = vector_angle(direction).to_degrees().round() as usize;
    let partition_size = 360 / len;

    (angle / partition_size) % len
}