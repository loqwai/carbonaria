
use bevy::prelude::*;

use crate::{components::LaserGunBullet, events::MoveEvent};

pub fn move_bullet(
    bullets: Query<(Entity, &Transform), With<LaserGunBullet>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    bullets.for_each(|(entity, transform)| {
        let velocity = transform.rotation * Vec3::X;
        let rotation = transform.rotation.clone();
        move_events.send(MoveEvent {
            who: entity,
            velocity,
            rotation,
        });
    });
}
