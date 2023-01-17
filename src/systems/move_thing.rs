use bevy::prelude::*;
use heron::prelude::*;

use crate::{events::MoveEvent, components::Speed};

pub fn move_thing(mut moveable: Query<(&mut Velocity, &Speed)>, speed_query: Query<(&Speed, &Parent)>, mut move_events: EventReader<MoveEvent>) {
    move_events.iter().for_each(|event| {
        match moveable.get_mut(event.target) {
            Ok((mut velocity, own_speed)) => {
                let mut entity_speed: f32 = own_speed.0;

                for (speed, parent) in speed_query.iter() {
                    if parent.id() != event.target.id() { continue }

                    entity_speed *= speed.0;
                }

                velocity.linear += event.velocity * entity_speed;
            }
            Err(_) => return,
        };
    });
}
