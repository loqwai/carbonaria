use bevy::prelude::*;

use crate::components::{Health, Math, Poison, TimeToLive};

pub fn poison(mut commands: Commands, q_poison: Query<(Entity, &Poison)>) {
    q_poison.for_each(|(entity, poison)| {
        println!("poisoning for {}", poison.0);
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_empty()
                .insert(Math::add(Health(-1)))
                .with_children(|parent| {
                    parent.spawn(Math::add(TimeToLive(10)));
                });
        });
    });
}
