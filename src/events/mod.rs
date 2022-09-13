use bevy::prelude::Entity;

#[derive(Debug)]
pub struct SwingStickEvent {
    pub stick: Entity,
}

pub struct StickHitEvent {
    pub stick: Entity,
    pub target: Entity,
}
