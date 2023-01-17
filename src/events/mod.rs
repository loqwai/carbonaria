use bevy::prelude::*;

#[derive(Debug)]
pub struct SwingStickEvent {
    pub stick: Entity,
}

pub struct StickHitEvent {
    pub stick: Entity,
    pub target: Entity,
}

pub struct ResetEvent {

}

pub struct MoveEvent {
   pub target: Entity,
   pub velocity: Vec3,
}
