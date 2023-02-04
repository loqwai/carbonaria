use bevy::prelude::*;

use crate::{events::MoveEvent, util::index_for_direction};

pub fn on_move_event_change_sprite_index(
    mut move_events: EventReader<MoveEvent>,
    mut sprites: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    move_events.iter().for_each(|event| {
        set_sprite_index(&mut sprites, &atlases, event);
    })
}

fn set_sprite_index(
    sprites: &mut Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    atlases: &Res<Assets<TextureAtlas>>,
    event: &MoveEvent,
) -> Option<()> {
    let (mut sprite, atlas_handle) = sprites.get_mut(event.who).ok()?;
    let atlas = atlases.get(atlas_handle)?;
    sprite.index = index_for_direction(event.direction, atlas.len());

    return None;
}
