use bevy::{prelude::*, sprite::SpriteBundle};
use bevy_rapier2d::prelude::*;

use crate::components::Chest;

const RADIUS: f32 = 64.0;

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub collider: Collider,
    pub sensor: Sensor,
    pub sprite_bundle: SpriteBundle,
    pub active_events: ActiveEvents,
}

impl ChestBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3, scale: f32, texture: &str, contents: Entity) -> ChestBundle {
        ChestBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            chest: Chest { contents: Some(contents), },
            collider: Collider::ball(RADIUS * scale),
            sensor: Sensor,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    ..Default::default()
                },
                texture: asset_server.load( format!("chests/{}.png", texture)),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
