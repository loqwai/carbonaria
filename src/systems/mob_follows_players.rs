use bevy::prelude::*;

use crate::{
    components::{Mob, Player}, events::MoveEvent,
};

pub fn mob_follows_players(
    q_player: Query<&Transform, With<Player>>,
    mut q_mob: Query<(Entity, &mut Transform), (With<Mob>, Without<Player>)>,
    mut move_events: EventWriter<MoveEvent>,
) {
    let player = q_player.get_single().unwrap();
    q_mob.for_each_mut(|(mob_entity, mut mob)| {
        let diff =  player.translation - mob.translation;
        let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
        mob.rotation = Quat::from_axis_angle(Vec3::Z, angle);

        move_events.send(MoveEvent{
            target: mob_entity,
            velocity: diff.normalize(),
        });
    });
}
