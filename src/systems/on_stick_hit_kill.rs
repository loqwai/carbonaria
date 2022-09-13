use bevy::prelude::*;

use crate::{components::Mob, events::StickHitEvent};

pub fn on_stick_hit_kill(
    mut commands: Commands,
    mut events: EventReader<StickHitEvent>,
    mobs: Query<Entity, With<Mob>>,
) {
    for event in events.iter() {
        match mobs.get(event.target) {
            Err(_) => continue,
            Ok(target) => commands.entity(target).despawn_recursive(),
        }
    }
}
