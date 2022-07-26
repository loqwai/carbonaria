use bevy::{
    math::{Quat, Vec3},
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use heron::{CollisionShape, RigidBody};

use crate::components::Wall;

type Position = (f32, f32);

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

pub enum WallTexture {
    Straight,
    Corner,
}

fn filename_for_wall_texture(texture: WallTexture) -> String {
    match texture {
        WallTexture::Straight => "wall-straight.png",
        WallTexture::Corner => "wall-corner.png",
    }
    .into()
}

impl WallBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture: WallTexture,
        (x, y): Position,
        rotation: f32,
        half_extends: Vec3,
    ) -> WallBundle {
        WallBundle {
            collision_shape: CollisionShape::Cuboid {
                half_extends,
                border_radius: None,
            },
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(filename_for_wall_texture(texture).as_str()),
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(16.0, 16.0, 0.0),
                border_radius: None,
            },
            sprite_bundle: Default::default(),
        }
    }
}
