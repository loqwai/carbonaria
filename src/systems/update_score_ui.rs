use bevy::prelude::*;

use crate::components::{Points, ScoreUI};

pub fn update_score_ui(mut q_score_ui: Query<&mut Text, With<ScoreUI>>, q_points: Query<&Points>) {
    for mut ui in q_score_ui.iter_mut() {
        match q_points.get_single() {
            Err(_) => continue,
            Ok(points) => {
                ui.sections[0].value = format!("Score     {}", points.0);
            }
        }
    }
}
