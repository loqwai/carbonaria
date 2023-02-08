use bevy::prelude::*;

use crate::{components::Speed, events::MoveEvent};

pub fn move_thing(
    mut moveable: Query<(&mut Transform, &Speed)>,
    mut move_events: EventReader<MoveEvent>,
) {
    move_events.iter().for_each(|event| {
        if event.direction == Vec3::default() {
            return; // We can't call normalize on a 0,0,0 vector
        }

        match moveable.get_mut(event.who) {
            Err(_) => return,
            Ok((mut transform, own_speed)) => {
                let entity_speed = own_speed.0;
                transform.translation +=
                    event.direction.normalize() * entity_speed;
            }
        };
    });
}
