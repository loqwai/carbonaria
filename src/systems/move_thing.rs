use bevy::prelude::*;
use heron::prelude::*;

use crate::events::MoveEvent;

pub fn move_thing(mut moveable: Query<&mut Velocity>, mut move_events: EventReader<MoveEvent>) {
    move_events.iter().for_each(|event| {
        match moveable.get_mut(event.target) {
            Ok(mut velocity) => velocity.linear += event.velocity,
            Err(_) => return,
        };
    });
}
