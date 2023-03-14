use bevy::prelude::*;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CameraType {
    /// 2d camera using sprites
    #[value(name = "2d")]
    Camera2d,
    /// 3d camera using 3d models
    #[value(name = "3d")]
    Camera3d,
}

#[derive(Resource, Parser)]
pub struct Config {
    #[arg(short, long, value_enum, default_value_t = CameraType::Camera2d)]
    pub camera_type: CameraType,

    /// dimensions defines the size of the room that is
    /// generated. The room will always be square, and will be
    /// `dimensions` tiles wide, `dimensions` tiles tall. If odd,
    /// it may become off by one.
    #[arg(short, long, default_value_t = 32)]
    pub dimensions: i16,
    /// tile_size indicates the size of each wall in pixels.
    #[arg(long, default_value_t = 64)]
    pub tile_size: i16,
    /// scale indicates how zoomed in we are. A scale of 1.0 will render
    /// the player at 256x256
    #[arg(short, long, default_value_t = 0.5)]
    pub scale: f32,
    /// camera_follow_interpolation defines how quickly the camera
    /// will follow the player. A value of 1.0 will cause the camera
    /// to move completely in-sync with. Lower values like 0.1 or 0.01
    /// cause the camera to have a more natural feeling "bungee" effect
    #[arg(long, default_value_t = 0.05)]
    pub camera_follow_interpolation: f32,
    #[arg(long, default_value_t = 100)]
    pub mob_spawn_interval: usize,
    #[arg(long, default_value_t = 3600)]
    pub mech_spawn_interval: usize,
    #[arg(long, default_value_t = 100)]
    pub powerup_spawn_interval: usize,
}
