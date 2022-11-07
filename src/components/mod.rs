mod room;
mod swing_stick_animation;
mod wall_type;
use bevy::prelude::*;

pub use room::Room;
pub use swing_stick_animation::SwingStickAnimation;
pub use wall_type::Port;
pub use wall_type::PortType;
pub use wall_type::QuarterRotation;
pub use wall_type::TileType;
pub use wall_type::WallType;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Points(pub usize);
impl Default for Points {
    fn default() -> Self {
        Points(0)
    }
}

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct Health(pub usize);
impl Default for Health {
    fn default() -> Self {
        Health(10)
    }
}

#[derive(Component)]
pub struct HealthTarget;

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
pub struct Compass;

#[derive(Component)]
pub struct Speedup;

#[derive(Component)]
pub struct Speed(pub f32);
impl Default for Speed {
    fn default() -> Self {
        Speed(40.0)
    }
}

#[derive(Component)]
pub struct Chest {
    pub contents: Option<Entity>,
}
impl Default for Chest {
    fn default() -> Self {
        Chest { contents: None }
    }
}
//https://github.com/bevyengine/bevy/discussions/3332
