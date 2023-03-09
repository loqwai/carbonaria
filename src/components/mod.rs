mod powerups;

use bevy::prelude::*;

pub use powerups::*;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Mech;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct SpriteAnimation {
    pub num_angles: usize,
    pub num_frames_per_angle: usize,
    pub frames_to_advance_per_tick: f32,

    /// index of the current angle. Must be set to a value less than num_angles
    pub current_angle: usize,
    /// The current animation frame independent of angle
    pub current_frame: f32,
}

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct AlwaysAnimate;

#[derive(Component)]
pub struct SpinMe(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Points(pub usize);
impl Default for Points {
    fn default() -> Self {
        Points(0)
    }
}

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct HealthTarget;

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
pub struct Compass;

#[derive(Component, Default)]
pub struct Chases;

// Chest stuff
#[derive(Component, Default)]
pub struct Pocket;

#[derive(Component)]
pub struct Chest {
    pub contents: Vec<Entity>,
}

#[derive(Component)]
pub struct AutoShoot;

#[derive(Component)]
pub struct MousePos;
//https://github.com/bevyengine/bevy/discussions/3332

#[derive(Component, Reflect, Debug)]
pub struct LaserGun {
    pub cooldown: usize,
    pub cooldown_max: usize,
    pub cooldown_rate: usize,
}
#[derive(Component, Reflect)]
pub struct Bullet;
#[derive(Component)]
pub struct Aimable;
#[derive(Component, Deref)]
pub struct Direction(pub Quat);

#[derive(Reflect, Resource)]
pub struct Tick(pub usize);

#[derive(Clone, Copy, Component)]
pub struct ActiveAmmo(pub AmmoType);
#[derive(Clone, Copy)]
pub enum AmmoType {
    Normal,
    Poison,
    RageQuit,
    Reverser,
}
