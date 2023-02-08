use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Health, Player, Pocket, Points, Speed, Team, RateOfFire};

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
    pub speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub team: Team,
    pub active_events: ActiveEvents,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub rigid_body: RigidBody,
    pub rate_of_fire: RateOfFire,
}

impl PlayerBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        scale: f32,
    ) -> PlayerBundle {
        let texture = asset_server.load("player-sprite-sheet.png");
        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        PlayerBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            axis_constraints: LockedAxes::all(),
            speed: Speed(0.0),
            collider: Collider::ball(RADIUS * scale),
            health: Health::default(),
            name: "player".into(),
            player: Player,
            pockets: Pocket,
            points: Points(0),
            rigid_body: RigidBody::Dynamic,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite{
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: 7,
                    ..Default::default()
                },
                ..Default::default()
            },
            sensor: Sensor,
            team: Team(0),
            rate_of_fire: RateOfFire(1),
        }
    }
}
