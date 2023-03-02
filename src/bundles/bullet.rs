use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Bullet, Direction, Speed, SpriteAnimation},
    constants::SCALE_FACTOR_3D,
    util::index_for_direction,
};

const BASE_SPEED: f32 = 20.0;
const RADIUS: f32 = 64.0;

#[derive(Bundle)]
pub struct BulletBundle {
    pub tag: Bullet,
    pub sprite: SpriteSheetBundle,
    pub sprite_animation: SpriteAnimation,
    pub direction: Direction,
    pub active_events: ActiveEvents,
    pub collider: Collider,
    pub sensor: Sensor,
    pub speed: Speed,
}

impl BulletBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        transform: Transform,
        texture_name: &str,
        scale: f32,
    ) -> BulletBundle {
        let texture = asset_server.get_handle(format!("sprites/bullets/{}.png", texture_name));
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_len = texture_atlas.len();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        BulletBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(RADIUS * scale),
            sensor: Sensor,
            direction: Direction(transform.rotation),
            speed: Speed(BASE_SPEED * scale),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: index_for_direction(transform.rotation * Vec3::X, texture_atlas_len),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(transform.translation),
                ..Default::default()
            },
            sprite_animation: SpriteAnimation {
                num_angles: 16,
                num_frames_per_angle: 1,
                frames_to_advance_per_tick: 1.0,
                current_angle: 0,
                current_frame: 0.0,
            },
            tag: Bullet,
        }
    }
}

#[derive(Bundle)]
pub struct BulletModelBundle {
    pub scene: SceneBundle,
}

impl BulletModelBundle {
    pub fn new(asset_server: &Res<AssetServer>, scale: f32, model_name: &str) -> BulletModelBundle {
        BulletModelBundle {
            scene: SceneBundle {
                scene: asset_server.load(format!("models/bullets/{}.gltf#Scene0", model_name)),
                transform: Transform {
                    scale: Vec3::splat(RADIUS * SCALE_FACTOR_3D * scale),
                    ..default()
                },
                ..default()
            },
        }
    }
}
