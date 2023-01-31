use bevy::{
    math::Vec3,
    prelude::*,
    sprite::{SpriteBundle, Sprite},
};
use bevy_rapier2d::prelude::*;

use crate::components::{Chases, Health, Mob, Pocket, Speed, Team};

const BASE_SPEED: f32 = 10.0;
const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub base_speed: Speed,
    pub pockets: Pocket,
    pub team: Team,
    pub health: Health,
    pub chases: Chases,
    pub sprite_bundle: SpriteBundle,
    pub axis_constraints: LockedAxes,
}

impl MobBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3, scale: f32) -> MobBundle {
        MobBundle {
            axis_constraints: LockedAxes::all(),
            base_speed: Speed(BASE_SPEED * scale),
            chases: Chases,
            collider: Collider::ball(RADIUS * scale),
            health: Health(2),
            mob: Mob,
            pockets: Pocket,
            rigid_body: RigidBody::Dynamic,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    ..Default::default()
                },
                texture: asset_server.load("mob.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            team: Team(1),
        }
    }
}
