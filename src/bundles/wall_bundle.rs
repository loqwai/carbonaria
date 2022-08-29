use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec3},
    prelude::{AssetServer, Bundle, Res, Transform},
    sprite::SpriteBundle,
};
use heron::RigidBody;

use crate::components::{TileType, Wall, WallType};

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
    match wall_type.tile_type {
        TileType::Empty => filename_for_wall_texture(WallTexture::Empty),
        TileType::Straight => filename_for_wall_texture(WallTexture::Straight),
        TileType::Corner => filename_for_wall_texture(WallTexture::Corner),
    }
}

fn rotation_for_wall_type(wall_type: &WallType) -> Quat {
    Quat::from_rotation_z(match wall_type.quarter_rotations {
        crate::components::QuarterRotation::Zero => 0.0,
        crate::components::QuarterRotation::One => PI / 2.0,
        crate::components::QuarterRotation::Two => PI,
        crate::components::QuarterRotation::Three => PI / -2.0,
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
            wall_type: WallType::empty(),
            rigid_body: RigidBody::Static,
            sprite_bundle: Default::default(),
        }
    }
}
