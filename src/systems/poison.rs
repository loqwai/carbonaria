use bevy::prelude::*;

use crate::components::{Health, Math, Poison, TimeToLive};

pub fn poison(mut commands: Commands, q_poison: Query<(Entity, &Poison)>) {
    q_poison.for_each(|(entity, poison)| {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(Math::add(Health(-1))).with_children(|p2| {
                p2.spawn(Math::add(TimeToLive(10)));
            });
        });
    });
}
