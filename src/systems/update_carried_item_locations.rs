use bevy::{ecs::system::QuerySingleError, prelude::*};

use crate::components::{Player, Stick};

/// currently will update the first stick item to follow the first player
/// item.
///
/// TODO: Make it work for any item/carrier combination
pub fn update_carried_item_locations(
    params: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Stick>>,
    )>,
) {
    if let Err(e) = fallible_update_carried_item_locations(params) {
        panic!("Error updating carried item locations: {}", e);
    }
}

fn fallible_update_carried_item_locations(
    mut params: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Stick>>,
    )>,
) -> Result<(), QuerySingleError> {
    let first = params.p0();
    let player = first.get_single()?;
    let player_loc = player.translation.clone();
    let stick_loc = player_loc + Vec3::new(56.0, 0.0, 0.0);

    let mut second = params.p1();
    let mut stick = second.get_single_mut()?;
    stick.translation = stick_loc;

    Ok(())
}
