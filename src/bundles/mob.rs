use bevy::{math::Vec3, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::components::{Chases, Health, Mob, Pocket, RateOfFire, Speed, SpriteAnimation, Team};

const RADIUS: f32 = 128.0;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub speed: Speed,
    pub pockets: Pocket,
    pub team: Team,
    pub health: Health,
    pub chases: Chases,
    pub scene: SceneBundle,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprite_animation: SpriteAnimation,
    pub axis_constraints: LockedAxes,
    pub rate_of_fire: RateOfFire,
}

impl MobBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        position: Vec3,
        scale: f32,
    ) -> MobBundle {
        let texture = asset_server.get_handle("sprites/units/mob.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        println!("position: {:?}", position);

        MobBundle {
            axis_constraints: LockedAxes::all(),
            speed: Speed::default(),
            chases: Chases,
            collider: Collider::ball(RADIUS * scale),
            health: Health(2.0),
            mob: Mob,
            pockets: Pocket,
            rigid_body: RigidBody::Dynamic,
            scene: SceneBundle {
                scene: asset_server.load("models/units/mob.gltf#Scene0"),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
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
            team: Team(1),
            rate_of_fire: RateOfFire(1.0),
        }
    }
}
