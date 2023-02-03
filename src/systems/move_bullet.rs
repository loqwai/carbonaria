use bevy::prelude::*;

use crate::{
    components::{Direction, LaserGunBullet},
    events::{MoveEvent},
};

pub fn move_bullet(
    bullets: Query<(Entity, &Direction), With<LaserGunBullet>>,
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
