use bevy::prelude::*;

pub struct DamagerHitEvent {
    pub damager: Entity,
    pub target: Entity,
}

pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

pub struct RotateEvent {
    pub who: Entity,
    pub rotation: Quat,
}

pub struct DespawnEvent {
    pub entity: Entity,
}
pub struct ShootEvent {
    pub gun: Entity,
}
