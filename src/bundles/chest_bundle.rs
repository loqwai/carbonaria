use bevy::{prelude::*, sprite::SpriteBundle};
use bevy_rapier2d::prelude::*;

use crate::components::Chest;

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub collider: Collider,
    pub sensor: Sensor,
    pub sprite_bundle: SpriteBundle,
    pub active_events: ActiveEvents,
}

impl ChestBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3, contents: Entity) -> ChestBundle {
        ChestBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("speed-chest.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            chest: Chest {
                contents: Some(contents),
            },
            ..Default::default()
        }
    }
}

impl Default for ChestBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            collider: Collider::ball(16.0),
            chest: Default::default(),
            sensor: Default::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
