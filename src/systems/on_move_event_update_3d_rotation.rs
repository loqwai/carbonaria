use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    events::MoveEvent,
    resources::{CameraType, Config},
};

pub fn on_move_event_update_3d_rotation(
    config: Res<Config>,
    mut move_events: EventReader<MoveEvent>,
    mut transforms: Query<&mut Transform>,
) {
    if config.camera_type != CameraType::Camera3d {
        return;
    }

    move_events.iter().for_each(|event| {
        update_3d_rotation(&mut transforms, event);
    })
}

fn update_3d_rotation(transforms: &mut Query<&mut Transform>, event: &MoveEvent) -> Option<()> {
    let direction = event.direction;
    let angle = (PI / 2.0) + direction.y.atan2(direction.x);

    let mut transform = transforms.get_mut(event.who).ok()?;
    transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

    Some(())
}
