use bevy::prelude::*;

use crate::events::StickHitEvent;

pub fn on_stick_hit_kill(mut commands: Commands, mut events: EventReader<StickHitEvent>) {
    for event in events.iter() {
        commands.entity(event.mob).despawn_recursive();
    }
}
