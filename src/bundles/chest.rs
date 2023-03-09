use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{AlwaysAnimate, Chest, SpinMe, SpriteAnimation},
    constants::SCALE_FACTOR_3D,
};

const RADIUS: f32 = 64.0;

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub collider: Collider,
    pub sensor: Sensor,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animation: SpriteAnimation,
    pub active_events: ActiveEvents,
    pub always_animate: AlwaysAnimate,
}

impl ChestBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        position: Vec3,
        scale: f32,
        texture: &str,
        tile_size: Vec2,
        // The number of tiles in the texture image
        (num_cols, num_rows): (usize, usize),
        num_frames: usize,
        contents: Vec<Entity>,
    ) -> ChestBundle {
        let texture = asset_server.get_handle(format!("sprites/chests/{}.png", texture));
        let texture_atlas =
            TextureAtlas::from_grid(texture, tile_size, num_cols, num_rows, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        ChestBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            chest: Chest { contents },
            collider: Collider::ball(RADIUS * scale),
            sensor: Sensor,
            always_animate: AlwaysAnimate,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(RADIUS * scale * 2.0, RADIUS * scale * 2.0)),
                    index: 0,
                    ..Default::default()
                },
                ..Default::default()
            },
            sprite_animation: SpriteAnimation {
                num_angles: 1,
                num_frames_per_angle: num_frames,
                frames_to_advance_per_tick: 0.5,
                current_angle: 0,
                current_frame: 0.0,
            },
        }
    }
}

#[derive(Bundle)]
pub struct ChestModelBundle {
    pub scene: SceneBundle,
    pub spin_me: SpinMe,
}

impl ChestModelBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        scale: f32,
        model_filename: &str,
    ) -> ChestModelBundle {
        ChestModelBundle {
            spin_me: SpinMe(1.0),
            scene: SceneBundle {
                scene: asset_server
                    .load(&format!("models/powerups/{}.gltf#Scene0", model_filename,)),
                transform: Transform {
                    scale: Vec3::splat(RADIUS * SCALE_FACTOR_3D * scale),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ChestFallbackModelBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility_bundle: VisibilityBundle,
}

impl ChestFallbackModelBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        scale: f32,
    ) -> ChestFallbackModelBundle {
        ChestFallbackModelBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: RADIUS * scale,
                    subdivisions: 5,
                }
                .into(),
            ),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
