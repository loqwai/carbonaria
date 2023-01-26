use bevy::prelude::*;

use crate::components::{Points, ScoreUI};

pub fn update_score_ui(mut q_score_ui: Query<&mut Text, With<ScoreUI>>, q_points: Query<&Points>) {
    q_score_ui.for_each_mut(|mut ui| match q_points.get_single() {
        Err(_) => return,
        Ok(points) => {
            ui.sections[0].value = format!("Score     {}", points.0);
        }
    })
}
