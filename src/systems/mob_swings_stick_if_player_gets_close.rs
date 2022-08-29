use bevy::prelude::*;

use crate::{
    components::{Mob, Player, Stick},
    events::SwingStickEvent,
};

pub fn mob_swings_stick_if_player_gets_close(
    q_mob: Query<(&Transform, &Children), With<Mob>>,
    q_player: Query<&Transform, With<Player>>,
    q_stick: Query<Entity, With<Stick>>,
    mut writer: EventWriter<SwingStickEvent>,
) {
    let player = q_player
        .get_single()
        .map_err(|e| format!("Failed to find player to try and hit with a stick: {}", e))
        .unwrap()
        .translation;

    // player.transform
    q_mob.for_each(|(transform, children)| {
        if !is_close(player, transform.translation) {
            return;
        }

        for &child in children.iter() {
            if let Ok(stick) = q_stick.get(child) {
                writer.send(SwingStickEvent { stick })
            }
        }
    });
}

pub fn is_close(t0: Vec3, t1: Vec3) -> bool {
    t0.distance(t1) < 64.0
}
