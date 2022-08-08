use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec3},
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use heron::RigidBody;

use crate::components::{Wall, WallType};

type Position = (i16, i16);

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub wall_type: WallType,
    pub rigid_body: RigidBody,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

#[derive(Clone, Copy, Debug)]
enum WallTexture {
    Empty,
    Straight,
    Corner,
}

fn filename_for_wall_texture(texture: WallTexture) -> String {
    match texture {
        WallTexture::Empty => "wall-empty.png",
        WallTexture::Straight => "wall-straight.png",
        WallTexture::Corner => "wall-corner.png",
    }
    .into()
}

fn texture_for_wall_type(wall_type: &WallType) -> String {
    match wall_type {
        WallType::Empty => filename_for_wall_texture(WallTexture::Empty),
        WallType::Vertical => filename_for_wall_texture(WallTexture::Straight),
        WallType::Horizontal => filename_for_wall_texture(WallTexture::Straight),
        WallType::TopLeftCorner => filename_for_wall_texture(WallTexture::Corner),
        WallType::TopRightCorner => filename_for_wall_texture(WallTexture::Corner),
        WallType::BottomRightCorner => filename_for_wall_texture(WallTexture::Corner),
        WallType::BottomLeftCorner => filename_for_wall_texture(WallTexture::Corner),
    }
}

fn rotation_for_wall_type(wall_type: &WallType) -> Quat {
    Quat::from_rotation_z(match wall_type {
        WallType::Empty => 0.0,
        WallType::Vertical => PI / 2.0,
        WallType::Horizontal => 0.0,
        WallType::TopLeftCorner => PI,
        WallType::TopRightCorner => PI / 2.0,
        WallType::BottomRightCorner => 0.0,
        WallType::BottomLeftCorner => PI / -2.0,
    })
}

impl WallBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        wall_type: &WallType,
        (x, y): Position,
    ) -> WallBundle {
        let tile_size: f32 = 64.0;
        let x: f32 = f32::from(x) * tile_size;
        let y: f32 = f32::from(y) * tile_size;

        WallBundle {
            wall_type: *wall_type,
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(&texture_for_wall_type(wall_type)),
                transform: Transform {
                    rotation: rotation_for_wall_type(wall_type),
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
            wall_type: WallType::Empty,
            rigid_body: RigidBody::Static,
            sprite_bundle: Default::default(),
        }
    }
}
