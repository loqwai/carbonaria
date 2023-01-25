use bevy::prelude::*;

use crate::{
    components::LaserGunBullet,
    events::{MoveEvent, RotateEvent},
};

pub fn move_bullet(
    bullets: Query<(Entity, &Transform), With<LaserGunBullet>>,
    mut move_events: EventWriter<MoveEvent>,
    mut rotate_events: EventWriter<RotateEvent>,
) {
    bullets.for_each(|(entity, transform)| {
        let direction = transform.rotation * Vec3::X;
        let rotation = transform.rotation.clone();

        move_events.send(MoveEvent {
            who: entity,
            direction,
        });

        rotate_events.send(RotateEvent {
            who: entity,
            rotation,
        });
    });
}
