use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Anchor, SpriteBundle},
};

use crate::components::Compass;

// use crate::components::{Stick, SwingStickAnimation};

#[derive(Bundle)]
pub struct CompassBundle {
    pub name: Name,
    pub compass: Compass,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl CompassBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
    ) -> CompassBundle {
        let name: Name = "stick".into();

        CompassBundle {
            name: name.clone(),
            compass: Compass{},
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("compass.png"),
                transform: Transform {
                    rotation: Quat::from_rotation_z(-PI),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(-20.0 / 32.0, 0.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
