use std::f32::consts::PI;

use bevy::{prelude::*, sprite::SpriteBundle};

use crate::{bundles::WallBundle, components::HitBox};

type Position = (f32, f32);

pub fn spawn_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_size: usize = 64;
    let num_tiles: i16 = 8;
    let end: i16 = (num_tiles / 2) * i16::try_from(tile_size).unwrap();
    let begin: i16 = end * -1;

    // bottom wall
    for i in (begin..end).step_by(tile_size) {
        let x = f32::from(i);
        let y = f32::from(i);
        let begin = f32::from(begin);
        let end = f32::from(end);

        match x {
            // we're talking corner
            x if x == begin => {
                commands.spawn_bundle(bottom_left_corner(&asset_server, (begin, begin)));
                commands.spawn_bundle(top_left_corner(&asset_server, (begin, end)));
                commands.spawn_bundle(top_right_corner(&asset_server, (end, end)));
                commands.spawn_bundle(bottom_right_corner(&asset_server, (end, begin)));
            }

            // normal straight wall section
            _ => {
                commands.spawn_bundle(horizontal_wall(&asset_server, (x, begin))); // bottom
                commands.spawn_bundle(horizontal_wall(&asset_server, (x, end))); // top
                commands.spawn_bundle(vertical_wall(&asset_server, (begin, y))); // left
                commands.spawn_bundle(vertical_wall(&asset_server, (end, y))); // right
            }
        }
    }
}

fn horizontal_wall(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        hitbox: HitBox::new(64.0, 32.0),
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-straight.png"),
            transform: Transform {
                // rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn vertical_wall(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        hitbox: HitBox::new(32.0, 64.0),
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-straight.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn top_left_corner(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-corner.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn top_right_corner(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-corner.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn bottom_right_corner(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-corner.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z(0.0),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn bottom_left_corner(asset_server: &Res<AssetServer>, (x, y): Position) -> WallBundle {
    WallBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("wall-corner.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / -2.0),
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
