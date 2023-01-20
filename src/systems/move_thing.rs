use bevy::prelude::*;

use crate::{components::Speed, events::MoveEvent};

pub fn move_thing(
    mut moveable: Query<(&mut Transform, &Speed)>,
    speed_query: Query<(&Speed, &Parent)>,
    mut move_events: EventReader<MoveEvent>,
) {
    move_events.iter().for_each(|event| {
        match moveable.get_mut(event.who) {
            Ok((mut transform, own_speed)) => {
                let mut entity_speed: f32 = own_speed.0;

                for (speed, parent) in speed_query.iter() {
                    if parent.get() != event.who {
                        continue;
                    }

                    entity_speed *= speed.0;
                }
                if event.velocity != Vec3::default() {
                    transform.translation += event.velocity.normalize() * entity_speed;
                }
                transform.rotation = event.rotation;
            }
            Err(_) => return,
        };
    });
}
