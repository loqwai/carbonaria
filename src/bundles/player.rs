use crate::components::SpriteAnimation;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Health, Player, Pocket, Points, RateOfFire, Speed, Team};

const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub collider: Collider,
    pub axis_constraints: LockedAxes,
    pub points: Points,
    pub health: Health,
    pub speed: Speed,
    pub name: Name,
    pub pockets: Pocket,
    pub active_events: ActiveEvents,
    // pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprite_animation: SpriteAnimation,
    pub rigid_body: RigidBody,
    pub scene: SceneBundle,
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
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                index: 7,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            sprite_animation: SpriteAnimation {
                num_angles: 16,
                num_frames_per_angle: 1,
                frames_to_advance_per_tick: 1.0,
                current_angle: 0,
                current_frame: 0.0,
            },
            scene: SceneBundle {
                scene: asset_server.load("models/units/player.gltf#Scene0"),
                // scene: asset_server.load("models/bullets/laser.gltf#Scene0"),
                ..default()
            },
            sensor: Sensor,
            team: Team(0),
            rate_of_fire: RateOfFire(1.0),
        }
    }
}
