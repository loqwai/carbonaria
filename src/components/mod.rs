mod room;
mod swing_stick_animation;
mod wall_type;

use bevy::prelude::*;

pub use room::Room;
pub use room::Tile;
pub use swing_stick_animation::SwingStickAnimation;
pub use wall_type::WallType;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Wall;
