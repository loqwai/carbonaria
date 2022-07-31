mod swing_stick_animation;

use bevy::prelude::*;

pub use swing_stick_animation::SwingStickAnimation;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Wall;
