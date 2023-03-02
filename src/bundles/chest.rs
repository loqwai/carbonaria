use bevy::{prelude::*, sprite::SpriteBundle};
use bevy_rapier2d::prelude::*;

use crate::components::Chest;

const RADIUS: f32 = 64.0;

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub collider: Collider,
    pub sensor: Sensor,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub sprite_bundle: SpriteBundle,
    pub active_events: ActiveEvents,
}

impl ChestBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec3,
        scale: f32,
        texture: &str,
        contents: Vec<Entity>,
    ) -> ChestBundle {
        ChestBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            chest: Chest { contents },
            collider: Collider::ball(RADIUS * scale),
            sensor: Sensor,
            mesh: meshes.add(
                shape::Icosphere {
                    radius: RADIUS * scale,
                    subdivisions: 5,
                }
                .into(),
            ),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    ..Default::default()
                },
                texture: asset_server.get_handle(format!("sprites/chests/{}.png", texture)),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
