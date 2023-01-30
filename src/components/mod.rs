use bevy::prelude::*;

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

#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
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

#[derive(Component, Default)]
pub struct Chases;

#[derive(Clone, Component, Reflect)]
pub struct Speed(pub f32);
impl Speed {
    pub fn fast() -> Speed {
        Speed(1.1)
    }

    pub fn slow() -> Speed {
        Speed(0.5)
    }
}
impl Default for Speed {
    fn default() -> Self {
        Speed(4.0)
    }
}

// Chest stuff
#[derive(Component, Default)]
pub struct Pocket;

#[derive(Component)]
pub struct Speedup;

#[derive(Component)]
pub struct Wallbreaker;
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
#[derive(Clone, Component, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Team(pub usize);
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

#[derive(Reflect, Resource)]
pub struct Tick(pub usize);
