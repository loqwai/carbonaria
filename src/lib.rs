pub mod bundles;
pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod systems;
pub mod util;

use bevy::prelude::States;

#[derive(Clone, Debug, Hash, Eq, PartialEq, States, Default)]
pub enum AppState {
    #[default]
    InGame,
}
