use crate::components::{AmmoCount, SpriteAnimation};
use crate::constants::SCALE_FACTOR_3D;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Health, Player, Pocket, Points, RateOfFire, Speed, Team};

const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub ammo: AmmoCount,
    pub collider: Collider,
    pub axis_constraints: LockedAxes,
    pub points: Points,
    pub health: Health,
    pub speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub active_events: ActiveEvents,
    pub sprite: SpriteSheetBundle,
    pub sprite_animation: SpriteAnimation,
    pub rigid_body: RigidBody,
    pub rate_of_fire: RateOfFire,
    pub sensor: Sensor,
    pub team: Team,
}

impl PlayerBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        scale: f32,
    ) -> PlayerBundle {
        let texture = asset_server.get_handle("sprites/units/player.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        PlayerBundle {
            ammo: AmmoCount(0),
            active_events: ActiveEvents::COLLISION_EVENTS,
            axis_constraints: LockedAxes::all(),
            speed: Speed(0.0),
            collider: Collider::ball(RADIUS * scale),
            health: Health(f32::MAX),
            name: "player".into(),
            player: Player,
            pockets: Pocket,
            points: Points(0),
            rigid_body: RigidBody::Dynamic,
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: 0,
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            },
            sprite_animation: SpriteAnimation {
                num_angles: 16,
                num_frames_per_angle: 1,
                frames_to_advance_per_tick: 1.0,
                current_angle: 0,
                current_frame: 0.0,
            },
            sensor: Sensor,
            team: Team(0),
            rate_of_fire: RateOfFire(1.0),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerModelBundle {
    pub scene: SceneBundle,
}

impl PlayerModelBundle {
    pub fn new(asset_server: &Res<AssetServer>, scale: f32) -> PlayerModelBundle {
        PlayerModelBundle {
            scene: SceneBundle {
                scene: asset_server.load("models/units/player.gltf#Scene0"),
                transform: Transform {
                    scale: Vec3::splat(RADIUS * SCALE_FACTOR_3D * scale),
                    ..default()
                },
                ..default()
            },
        }
    }
}
