use bevy::prelude::*;

use crate::{components::Player, AppState};

pub fn on_click_and_no_player_reset(
    players: Query<&Player>,
    buttons: Res<Input<MouseButton>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if buttons.pressed(MouseButton::Left) && players.is_empty() {
        app_state.restart().unwrap()
    }
}
