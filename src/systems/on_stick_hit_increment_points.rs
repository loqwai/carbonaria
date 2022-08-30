use bevy::prelude::*;

use crate::components::{Player, Points};

pub fn on_stick_hit_increment_points(mut query: Query<&mut Points, With<Player>>) {
    for mut points in query.iter_mut() {
        points.0 += 1;
    }
}
