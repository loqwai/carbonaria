use bevy::{prelude::Bundle, sprite::SpriteBundle};

use crate::components::Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: Default::default(),
        }
    }
}
