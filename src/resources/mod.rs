use bevy::core::Timer;

pub struct MobSpawnTimer(pub Timer);

pub struct Config {
    /// dimensions defines the size of the room that is
    /// generated. The room will always be square, and will be
    /// `dimensions` tiles wide, `dimensions` tiles tall. If odd,
    /// it may become off by one.
    pub dimensions: i16,
}
