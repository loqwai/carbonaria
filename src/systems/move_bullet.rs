use bevy::prelude::*;

use crate::{
    components::{Direction, Bullet},
    events::{MoveEvent},
};

pub fn move_bullet(
    bullets: Query<(Entity, &Direction), With<Bullet>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    bullets.for_each(|(entity, direction)| {
        let direction = direction.0 * Vec3::X;

        move_events.send(MoveEvent {
            who: entity,
            direction,
        });
    });
}
