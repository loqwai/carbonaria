use bevy::prelude::*;

use crate::{
    components::{Mob, Player},
    resources::Config,
};

pub fn mob_follows_players(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    mut q_mob: Query<&mut Transform, (With<Mob>, Without<Player>)>,
) {
    let player = q_player.get_single().unwrap();
    q_mob.for_each_mut(|mut mob| {
        let diff =  mob.translation - player.translation;
        let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
        mob.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        mob.translation -= diff.normalize();
    });
}
