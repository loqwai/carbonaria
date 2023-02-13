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
                texture: asset_server.get_handle("compass.png"),
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -5.0 / 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
