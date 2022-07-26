use bevy::{math::Vec2, prelude::Component};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct HitBox(Vec2);

impl HitBox {
    pub fn new(x: f32, y: f32) -> HitBox {
        HitBox(Vec2::new(x, y))
    }

    pub fn clone_vec2(&self) -> Vec2 {
        self.0.clone()
    }
}

impl Into<Vec2> for HitBox {
    fn into(self) -> Vec2 {
        self.0
    }
}
