use crate::components::{Health, HealthTarget};
use bevy::prelude::*;

pub fn update_health_ui(
    mut q_health_ui: Query<(&mut Text, &Parent), With<HealthTarget>>,
    q_healths: Query<&Health>,
) {
    for (mut health_text, parent) in q_health_ui.iter_mut() {
        let target_health = q_healths.get(parent.get()).unwrap(); // Maybe Bevy 0.8+ only?

        health_text.sections[0].value = format!("{}", target_health.0.round() as i32);
    }
}
