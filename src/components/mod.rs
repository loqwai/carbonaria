mod powerups;

use bevy::prelude::*;

pub use powerups::*;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stick;

#[derive(Component)]
pub struct Wall;

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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Chest {
    pub contents: Option<Entity>,
}
impl Default for Chest {
    fn default() -> Self {
        Chest { contents: None }
    }
}
#[derive(Component)]
pub struct MousePos;
//https://github.com/bevyengine/bevy/discussions/3332

#[derive(Component, Reflect)]
pub struct LaserGun {
    pub cooldown: usize,
    pub cooldown_max: usize,
}
#[derive(Component, Reflect)]
pub struct LaserGunBullet;

#[derive(Component, Reflect)]
pub struct Damage(pub usize);

#[derive(Component)]
pub struct Aimable;

#[derive(Component, Reflect)]
pub struct TimeToLive(pub usize);

#[derive(Component, Deref)]
pub struct Direction(pub Quat);


#[derive(Reflect, Resource)]
pub struct Tick(pub usize);
