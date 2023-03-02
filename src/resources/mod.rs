mod config;

use bevy::prelude::{Deref, DerefMut, Resource};
use rand::SeedableRng;

pub use config::*;

#[derive(Deref, DerefMut, Resource)]
pub struct SmallRng(pub rand::rngs::SmallRng); // Bevy 0.9.0+ requires all resources to derive Resource

impl SmallRng {
    pub fn from_entropy() -> SmallRng {
        SmallRng(rand::rngs::SmallRng::from_entropy())
    }
}
