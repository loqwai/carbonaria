use bevy::prelude::*;

use crate::components::Exit;

#[derive(Bundle)]
pub struct ExitBundle {
    pub exit: Exit,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl ExitBundle {
    pub fn new(asset_server: &Res<AssetServer>, position: Vec3) -> ExitBundle {
        ExitBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("exit.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for ExitBundle {
    fn default() -> Self {
        Self {
            exit: Exit {},
            sprite_bundle: Default::default(),
        }
    }
}
