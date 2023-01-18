use bevy::{prelude::*, sprite::SpriteBundle};
use bevy_rapier2d::prelude::*;

use crate::components::{Health, Player, Pocket, Points, Speed, Team};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: LockedAxes,
    pub points: Points,
    pub health: Health,
    pub speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub team: Team,
    pub sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::ball(16.0),
            velocity: Default::default(),
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 0.0,
            },
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            health: Default::default(),
            sprite_bundle: Default::default(),
            points: Default::default(),
            speed: Default::default(),
            pockets: Pocket,
            team: Team(0),
            name: "player".into(), //probably not something we want to do in the future. But nice for the debugger
        }
    }
}
