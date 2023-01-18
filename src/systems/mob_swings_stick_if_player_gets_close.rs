use bevy::prelude::*;

use crate::{
    components::{Team, Chases, Stick},
    events::SwingStickEvent,
};

pub fn mob_swings_stick_if_player_gets_close(
    chasers: Query<(&Team, &Transform, &Children), With<Chases>>,
    targets: Query<(&Team, &Transform)>,
    sticks: Query<Entity, With<Stick>>,
    mut writer: EventWriter<SwingStickEvent>,
) {
    chasers.for_each(|(chaser_team, chaser_transform, children)| {
        match targets.iter().find(|(team, _)| team != &chaser_team) {
            None => return,
            Some((_, target_transform)) => {
                if !is_close(chaser_transform.translation, target_transform.translation) {
                    return;
                }
                for &child in children.iter() {
                    // get the health of each child unit
                    if let Ok(stick) = sticks.get(child) {

                        writer.send(SwingStickEvent { stick });
                    }
                }
            }
        }
    });
}

pub fn is_close(t0: Vec3, t1: Vec3) -> bool {
    t0.distance(t1) < 64.0
}
