use bevy::prelude::{Deref, DerefMut, Resource};
use rand::SeedableRng;

#[derive(Resource)]
pub struct Config {
    /// dimensions defines the size of the room that is
    /// generated. The room will always be square, and will be
    /// `dimensions` tiles wide, `dimensions` tiles tall. If odd,
    /// it may become off by one.
    pub dimensions: i16,
    /// tile_size indicates the size of each wall in pixels.
    pub tile_size: i16,
    /// camera_follow_interpolation defines how quickly the camera
    /// will follow the player. A value of 1.0 will cause the camera
    /// to move completely in-sync with. Lower values like 0.1 or 0.01
    /// cause the camera to have a more natural feeling "bungee" effect
    pub camera_follow_interpolation: f32,
    pub mob_spawn_interval: usize,
    pub powerup_spawn_interval: usize,
}

#[derive(Deref, DerefMut, Resource)]
pub struct SmallRng(pub rand::rngs::SmallRng); // Bevy 0.9.0+ requires all resources to derive Resource

impl SmallRng {
    pub fn from_entropy() -> SmallRng {
        SmallRng(rand::rngs::SmallRng::from_entropy())
    }
}
