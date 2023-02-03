use bevy::{
    math::Vec3,
    prelude::*,
    sprite::SpriteSheetBundle,
};
use bevy_rapier2d::prelude::*;

use crate::components::{Chases, Health, Mob, Pocket, Speed, Team};

const BASE_SPEED: f32 = 10.0;
const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub base_speed: Speed,
    pub pockets: Pocket,
    pub team: Team,
    pub health: Health,
    pub chases: Chases,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub axis_constraints: LockedAxes,
}

impl MobBundle {
    pub fn new(
        asset_server: &Res<AssetServer>, 
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        position: Vec3, 
        scale: f32,
    ) -> MobBundle {
        let texture = asset_server.load("mob-sprite-sheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 2, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        MobBundle {
            axis_constraints: LockedAxes::all(),
            base_speed: Speed(BASE_SPEED * scale),
            chases: Chases,
            collider: Collider::ball(RADIUS * scale),
            health: Health(2),
            mob: Mob,
            pockets: Pocket,
            rigid_body: RigidBody::Dynamic,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform { translation: position, ..Default::default()  },
                sprite: TextureAtlasSprite{
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: 7,
                    ..Default::default()
                },
                ..Default::default()
            },
            team: Team(1),
        }
    }
}
