use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteBundle},
};
use bevy_rapier2d::prelude::*;

use crate::components::{Stick, SwingStickAnimation};

#[derive(Bundle)]
pub struct StickBundle {
    pub name: Name,
    pub stick: Stick,
    pub collider: Collider,
    pub animation_player: AnimationPlayer,
    pub animation: SwingStickAnimation,
    pub sprite_bundle: SpriteBundle,
    sensor: Sensor,
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
                    anchor: Anchor::Custom(Vec2::new(-20.0 / 32.0, 0.0)),
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
            sensor: Sensor,
            collider: Collider::convex_hull(&vec![
                Vec2::new(36.0, 2.0),
                Vec2::new(44.0, 2.0),
                Vec2::new(44.0, -2.0),
                Vec2::new(36.0, -2.0),
            ])
            .unwrap(),
            animation_player: Default::default(),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
            animation: Default::default(),
        }
    }
}
