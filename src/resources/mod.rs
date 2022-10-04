use bevy::time::Timer;

pub struct MobSpawnTimer(pub Timer);

pub struct Config {
    /// dimensions defines the size of the room that is
    /// generated. The room will always be square, and will be
    /// `dimensions` tiles wide, `dimensions` tiles tall. If odd,
    /// it may become off by one.
    pub dimensions: i16,
    /// camera_follow_interpolation defines how quickly the camera
    /// will follow the player. A value of 1.0 will cause the camera
    /// to move completely in-sync with. Lower values like 0.1 or 0.01
    /// cause the camera to have a more natural feeling "bungee" effect
    pub camera_follow_interpolation: f32,
}
