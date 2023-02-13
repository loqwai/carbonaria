use bevy::prelude::*;
use crate::{components::{Poison, Math}};

pub fn attach_poison(
    mut commands: Commands,
    q_time_to_live: Query<Without<Poison>>,
    mut q_poison_powerup: Query<&Parent, With<Math<Poison>>>,
) {

    q_poison_powerup.for_each_mut(|parent| {
        if let Ok(_) = q_time_to_live.get(parent.get()) {
            println!("attaching poison");
            commands.entity(parent.get()).insert(Poison::default());
        }
    });
}
