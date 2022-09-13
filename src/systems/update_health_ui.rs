use crate::components::{Health, HealthTarget};
use bevy::{prelude::*, render::primitives::Frustum};

pub fn update_health_ui(
    mut q_health_ui: Query<(&mut Text, &mut HealthTarget), Without<Camera>>,
    q_healths: Query<(&Health, &Transform), (Without<HealthTarget>, Without<Camera>)>,
    mut q_cameras: Query<&mut Transform, (With<Camera>, Without<HealthTarget>, Without<Frustum>)>,
) {
    for (mut text, target) in q_health_ui.iter_mut() {
        let (health, target_transform) = q_healths.get(target.0).unwrap();
        let mut camera = q_cameras.get_mut(target.1).unwrap();
        text.sections[0].value = format!("{}", health.0);
        camera.translation = -target_transform.translation;
        camera.translation.z = 100.0;
    }
}
