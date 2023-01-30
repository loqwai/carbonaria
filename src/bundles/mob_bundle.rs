use bevy::{
    math::Vec3,
    prelude::*,
    sprite::{SpriteBundle, Sprite},
};
use bevy_rapier2d::prelude::*;

use crate::components::{Chases, Health, Mob, Pocket, Speed, Team};

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub base_speed: Speed,
    pub pockets: Pocket,
    pub team: Team,
    pub health: Health,
    pub chases: Chases,
    pub sprite_bundle: SpriteBundle,
    pub axis_constraints: LockedAxes,
}

impl MobBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3) -> MobBundle {
        MobBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                texture: asset_server.load("mob.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for MobBundle {
    fn default() -> Self {
        Self {
            mob: Mob,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(128.0),
            velocity: Default::default(),
            base_speed: Speed::fast(),
            pockets: Default::default(),
            sprite_bundle: Default::default(),
            axis_constraints: LockedAxes::all(),
            team: Team(1),
            health: Default::default(),
            chases: Default::default(),
        }
    }
}
