use bevy::{math::Vec3, prelude::*, sprite::SpriteSheetBundle};
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Chases, Health, Mech, Pocket, RateOfFire, Speed, SpriteAnimation, Team},
    constants::SCALE_FACTOR_3D,
};

const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct MechBundle {
    pub mech: Mech,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub speed: Speed,
    pub pockets: Pocket,
    pub team: Team,
    pub health: Health,
    pub chases: Chases,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animation: SpriteAnimation,
    pub axis_constraints: LockedAxes,
    pub rate_of_fire: RateOfFire,
}

impl MechBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        position: Vec3,
        scale: f32,
    ) -> MechBundle {
        let texture = asset_server.get_handle("sprites/units/mech.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(128.0, 128.0), 28, 28, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        MechBundle {
            axis_constraints: LockedAxes::all(),
            speed: Speed::default(),
            chases: Chases,
            collider: Collider::ball(RADIUS * scale),
            health: Health(2.0),
            mech: Mech,
            pockets: Pocket,
            rigid_body: RigidBody::Dynamic,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: 7,
                    ..Default::default()
                },
                ..Default::default()
            },
            sprite_animation: SpriteAnimation {
                num_angles: 16,
                num_frames_per_angle: 47,
                frames_to_advance_per_tick: 0.5,
                current_angle: 0,
                current_frame: 0.0,
            },
            team: Team(1),
            rate_of_fire: RateOfFire(1.0),
        }
    }
}

#[derive(Bundle)]
pub struct MechModelBundle {
    pub scene: SceneBundle,
}

impl MechModelBundle {
    pub fn new(asset_server: &Res<AssetServer>, scale: f32) -> MechModelBundle {
        MechModelBundle {
            scene: SceneBundle {
                scene: asset_server.load("models/units/mech.gltf#Scene0"),
                transform: Transform {
                    scale: Vec3::splat(RADIUS * SCALE_FACTOR_3D * scale),
                    ..default()
                },
                ..default()
            },
        }
    }
}
