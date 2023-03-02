use bevy::prelude::*;

use crate::{
    components::{Aimable, MousePos, Player},
    events::RotateEvent,
    util::look_at_target,
};

pub fn player_aimables_aim_at_cursor(
    mouses: Query<&Transform, With<MousePos>>,
    players: Query<&Children, With<Player>>,
    aimables: Query<(Entity, &GlobalTransform), With<Aimable>>,
    mut rotate_events: EventWriter<RotateEvent>,
) {
    mouses.iter().for_each(|mouse| {
        players.for_each(|player_children| {
            player_children
                .iter()
                .filter_map(|&child| aimables.get(child).ok())
                .for_each(|(aimable, aimable_transform)| {
                    let (rotation, _) =
                        look_at_target(aimable_transform.translation(), mouse.translation);

                    rotate_events.send(RotateEvent {
                        who: aimable,
                        rotation,
                    })
                })
        })
    })
}
