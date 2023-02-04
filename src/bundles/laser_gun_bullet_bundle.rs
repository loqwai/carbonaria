use bevy::{
    prelude::*,
    sprite::SpriteSheetBundle,
};
use bevy_rapier2d::prelude::*;

use crate::{components::{Direction, LaserGunBullet, Speed,Damage, Health, TimeToLive}, util::index_for_direction};

const BASE_SPEED: f32 = 20.0;
const RADIUS: f32 = 64.0;

#[derive(Bundle)]
pub struct LaserGunBulletBundle {
    pub name: Name,
    pub tag: LaserGunBullet,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub collider: Collider,
    pub direction: Direction,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub speed: Speed,
    pub damage: Damage,
    pub health: Health,
    pub time_to_live: TimeToLive,
}

impl LaserGunBulletBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        transform: &GlobalTransform,
        scale: f32,
    ) -> LaserGunBulletBundle {
        let transform = transform.compute_transform();
        let texture = asset_server.load("bullet-sprite-sheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_len = texture_atlas.len();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        LaserGunBulletBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(RADIUS * scale),
            damage: Damage(1),
            direction: Direction(transform.rotation),
            health: Health(1),
            name: "laser gun bullet".into(),
            sensor: Sensor,
            speed: Speed(BASE_SPEED * scale),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: transform.translation,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite{
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: index_for_direction(transform.rotation * Vec3::X, texture_atlas_len),
                    ..Default::default()
                },
                ..Default::default()
            },
            tag: LaserGunBullet,
            time_to_live: TimeToLive(100),
        }
    }
}
