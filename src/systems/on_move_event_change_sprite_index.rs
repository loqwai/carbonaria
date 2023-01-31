use std::f32::consts::PI;

use bevy::prelude::*;

use crate::events::MoveEvent;

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
    let x = direction.x;
    let y = direction.y;
    
    let mut angle_r = x.atan2(y);

    if angle_r < 0.0 {
        angle_r += PI * 2.0;
    }
    
    let angle = angle_r.to_degrees().round() as usize;

    (angle / 45) % 8 // 360 / 8 
}