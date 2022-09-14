use crate::components::{Health, HealthTarget};
use bevy::prelude::*;

pub fn update_health_ui(
    mut q_health_ui: Query<(&mut Text, &mut Transform, &HealthTarget)>,
    q_healths: Query<(&Health, &Transform), Without<HealthTarget>>,
) {
    for (mut health_text, mut health_transform, target) in q_health_ui.iter_mut() {
        let (target_health, target_transform) = q_healths.get(target.0).unwrap();

        health_text.sections[0].value = format!("{}", target_health.0);
        health_transform.translation = target_transform.translation;
    }
}
