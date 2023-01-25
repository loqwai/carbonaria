use bevy::prelude::*;

#[derive(Debug)]
pub struct SwingStickEvent {
    pub stick: Entity,
}

pub struct StickHitEvent {
    pub stick: Entity,
    pub target: Entity,
}
pub struct DamagerHitEvent {
    pub damager: Entity,
    pub target: Entity,
}

pub struct ResetEvent {}

pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

pub struct RotateEvent {
    pub who: Entity,
    pub rotation: Quat,
}
