use bevy::prelude::*;

#[derive(Event)]
pub struct DamagerHitEvent {
    pub damager: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

#[derive(Event)]
pub struct RotateEvent {
    pub who: Entity,
    pub rotation: Quat,
}

#[derive(Event)]
pub struct DespawnEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct ShootEvent {
    pub gun: Entity,
}
