use bevy::prelude::*;

use crate::{components::Wall, resources::PlayerResource};

pub fn detect_player_wall_collisions(
    player: Res<PlayerResource>,
    transforms: Query<&mut Transform>,
    walls: Query<(&Transform, &Wall)>,
) {
    if let Err(e) = fallible_detect_player_wall_collisions(player, transforms, walls) {
        panic!("Error detecting collisions: {}", e);
    }
}

fn fallible_detect_player_wall_collisions(
    _player: Res<PlayerResource>,
    _transforms: Query<&mut Transform>,
    _walls: Query<(&Transform, &Wall)>,
) -> Result<(), String> {
    Ok(())
}
