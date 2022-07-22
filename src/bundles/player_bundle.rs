use bevy::{prelude::Bundle, sprite::SpriteBundle};

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_bundle: Default::default(),
        }
    }
}
