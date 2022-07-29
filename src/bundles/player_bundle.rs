use bevy::{
    prelude::{AssetServer, Bundle, Res},
    sprite::SpriteBundle,
};
use heron::{CollisionShape, Damping, RigidBody, RotationConstraints, Velocity};

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: RotationConstraints,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
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
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: 32.0 },
            velocity: Default::default(),
            damping: Damping::from_linear(10.0),
            rotation_constraints: RotationConstraints {
                allow_x: false,
                allow_y: false,
                allow_z: false,
            },
            sprite_bundle: Default::default(),
        }
    }
}
