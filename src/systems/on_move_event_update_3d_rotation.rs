use std::f32::consts::PI;

use bevy::prelude::*;

use crate::events::MoveEvent;

pub fn on_move_event_update_3d_rotation(
    mut move_events: EventReader<MoveEvent>,
    childrens: Query<&Children>,
    mut models: Query<&mut Transform, With<Handle<Scene>>>,
) {
    move_events.iter().for_each(|event| {
        update_3d_rotation(&childrens, &mut models, event);
    })
}

fn update_3d_rotation(
    childrens: &Query<&Children>,
    models: &mut Query<&mut Transform, With<Handle<Scene>>>,
    event: &MoveEvent,
) -> Option<()> {
    let direction = event.direction;
    let angle = (PI / 2.0) + direction.y.atan2(direction.x);

    let children = childrens.get(event.who).ok()?;
    for child in children {
        let Ok(mut transform) = models.get_mut(*child) else { continue; };

        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }

    Some(())
}
