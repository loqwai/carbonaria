use bevy::prelude::{ResMut};

use crate::components::Tick;
pub fn count_ticks(
    mut ticker: ResMut<Tick>,
) {
  ticker.0 += 1;
}
