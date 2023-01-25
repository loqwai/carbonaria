use bevy::prelude::*;

use crate::events::RotateEvent;

pub fn rotate_thing(
    mut things: Query<&mut Transform>,
    mut rotate_events: EventReader<RotateEvent>,
) {
    rotate_events
        .iter()
        .for_each(|rotate_event| match things.get_mut(rotate_event.who) {
            Err(_) => return, // Maybe this thing died before we could rotate it?
            Ok(mut thing) => {
                thing.rotation = rotate_event.rotation;
            }
        })
}
