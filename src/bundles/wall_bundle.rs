use bevy::{prelude::Bundle, sprite::SpriteBundle};

use crate::components::{HitBox, Wall};

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub hitbox: HitBox,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            hitbox: HitBox::new(0.0, 0.0),
            sprite_bundle: Default::default(),
        }
    }
}
