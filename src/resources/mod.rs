use bevy::prelude::Entity;

#[derive(Default)]
pub struct PlayerResource {
    pub entity: Option<Entity>,
}
