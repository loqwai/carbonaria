use bevy::{prelude::Bundle, sprite::SpriteBundle};

use crate::components::{HitBox, Player};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub hitbox: HitBox,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            hitbox: HitBox::new(64.0, 64.0),
            sprite_bundle: Default::default(),
        }
    }
}
