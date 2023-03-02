pub mod bundles;
pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod systems;
pub mod util;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    InGame,
}
