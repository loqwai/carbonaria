use crate::components::{Math, Poison};
use bevy::prelude::*;

pub fn attach_poison(
    mut commands: Commands,
    q_no_poison: Query<Without<Poison>>,
    mut q_poison_powerup: Query<&Parent, With<Math<Poison>>>,
) {
    q_poison_powerup.for_each_mut(|parent| {
        if let Ok(_) = q_no_poison.get(parent.get()) {
            commands.entity(parent.get()).insert(Poison::default());
        }
    });
}
