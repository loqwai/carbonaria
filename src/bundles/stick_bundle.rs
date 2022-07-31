use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteBundle},
};
use heron::{CollisionShape, RigidBody};

use crate::components::{Stick, SwingStickAnimation};

#[derive(Bundle)]
pub struct StickBundle {
    pub name: Name,
    pub stick: Stick,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub animation_player: AnimationPlayer,
    pub animation: SwingStickAnimation,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl StickBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        mut animations: ResMut<Assets<AnimationClip>>,
    ) -> StickBundle {
        let name: Name = "stick".into();

        StickBundle {
            name: name.clone(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("stick.png"),
                transform: Transform {
                    rotation: Quat::from_rotation_z(-PI),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-40.0 / 32.0, 0.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            animation: SwingStickAnimation::new(&mut animations, name),
            ..Default::default()
        }
    }
}

impl Default for StickBundle {
    fn default() -> Self {
        Self {
            name: "stick".into(),
            stick: Stick,
            rigid_body: RigidBody::KinematicPositionBased,
            collision_shape: CollisionShape::ConvexHull {
                points: vec![
                    Vec3::new(56.0, 4.0, 0.0),
                    Vec3::new(84.0, 4.0, 0.0),
                    Vec3::new(84.0, -4.0, 0.0),
                    Vec3::new(56.0, -4.0, 0.0),
                ],
                border_radius: None,
            },
            animation_player: Default::default(),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
            animation: Default::default(),
        }
    }
}
