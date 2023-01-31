use bevy::{prelude::*, sprite::SpriteBundle};
use bevy_rapier2d::prelude::*;

use crate::components::{Health, Player, Pocket, Points, Speed, Team};

const BASE_SPEED: f32 = 16.0;
const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sensor: Sensor,
    pub collider: Collider,
    pub axis_constraints: LockedAxes,
    pub points: Points,
    pub health: Health,
    pub base_speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub team: Team,
    pub active_events: ActiveEvents,
    pub sprite_bundle: SpriteBundle,
    pub rigid_body: RigidBody,
}

impl PlayerBundle {
    pub fn new(asset_server: &Res<AssetServer>, scale: f32) -> PlayerBundle {
        PlayerBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            axis_constraints: LockedAxes::all(),
            base_speed: Speed(BASE_SPEED * scale),
            collider: Collider::ball(RADIUS * scale),
            health: Health(100),
            name: "player".into(),
            player: Player,
            pockets: Pocket,
            points: Points(0),
            rigid_body: RigidBody::Dynamic,
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("player.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            sensor: Sensor,
            team: Team(0),
        }
    }
}
