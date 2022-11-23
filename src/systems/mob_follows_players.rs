use bevy::prelude::*;

use crate::{components::{Player, Mob}, resources::Config};

pub fn mob_follows_players(
    config: Res<Config>,
    q_player: Query<&Transform, With<Player>>,
    mut q_mob: Query<&mut Transform, (With<Mob>, Without<Player>)>,
) {
    let player = q_player.get_single().unwrap();
    let target = player.translation;
    q_mob.for_each_mut(|mut transform| {
        let mob_location = transform.translation;
        let direction = (target - mob_location).normalize();
        transform.translation += direction * config.mob_speed;
    });
}
