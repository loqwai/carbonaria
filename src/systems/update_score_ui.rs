use bevy::prelude::*;

use crate::components::{Points, ScoreUI};

pub fn update_score_ui(mut q_score_ui: Query<&mut Text, With<ScoreUI>>, q_points: Query<&Points>) {
    let mut ui = q_score_ui.get_single_mut().unwrap();
    let points = q_points.get_single().unwrap();

    ui.sections[0].value = format!("Score     {}", points.0);
}
