use bevy::{
    prelude::{AssetServer, Bundle, Res},
    sprite::SpriteBundle,
};
use heron::{CollisionShape, RigidBody, Velocity};

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,

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
            sprite_bundle: Default::default(),
        }
    }
}
