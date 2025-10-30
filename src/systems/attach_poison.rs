use crate::components::{Math, Poison};
use bevy::prelude::*;

pub fn attach_poison(
    mut commands: Commands,
    q_no_poison: Query<Entity, Without<Poison>>,
    q_poison_powerup: Query<&Parent, With<Math<Poison>>>,
) {
    for parent in q_poison_powerup.iter() {
        if q_no_poison.get(parent.get()).is_ok() {
            commands.entity(parent.get()).insert(Poison::default());
        }
    }
}
