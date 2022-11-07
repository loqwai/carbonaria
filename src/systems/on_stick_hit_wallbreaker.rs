use bevy::prelude::*;

use crate::{
    components::{Stick, Wall, Wallbreaker},
    events::StickHitEvent,
};

pub fn on_stick_hit_wallbreaker(
    mut commands: Commands,
    mut events: EventReader<StickHitEvent>,
    q_wallbreaker: Query<&Parent, With<Wallbreaker>>,
    q_stick: Query<&Parent, With<Stick>>,
    q_walls: Query<Entity, With<Wall>>,
) {
    for wallbreaker_parent in q_wallbreaker.iter() {
        for event in events.iter() {
            match q_stick.get(event.stick) {
                Err(_) => continue,
                Ok(stick_parent) => {
                    if stick_parent == wallbreaker_parent {
                        match q_walls.get(event.target) {
                            Err(_) => continue,
                            Ok(wall_entity) => {
                                commands.entity(wall_entity).despawn_recursive();
                            }
                        }
                    }
                }
            };
        }
    }
}
