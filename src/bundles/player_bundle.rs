use bevy::{prelude::*, sprite::SpriteBundle};
use heron::{CollisionShape, Damping, RigidBody, RotationConstraints, Velocity};

use crate::components::{Health, Player, Points, Speed, Pocket, Team,};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: RotationConstraints,
    pub points: Points,
    pub health: Health,
    pub speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub team: Team,
    #[bundle]
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
            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Sphere { radius: 16.0 },
            velocity: Default::default(),
            damping: Damping::from_linear(10.0),
            rotation_constraints: RotationConstraints {
                allow_x: false,
                allow_y: false,
                allow_z: false,
            },
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
