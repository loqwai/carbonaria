use bevy::prelude::*;

use crate::components::{Player, Stick, SwingStickAnimation};

pub fn on_click_mark_stick_swinging(
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    q_player: Query<&Children, With<Player>>,
    q_stick: Query<Entity, (With<Stick>, Without<SwingStickAnimation>)>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let children = q_player
        .get_single()
        .map_err(|e| format!("Failed to find player to swing stick: {}", e))
        .unwrap();

    for &child in children.iter() {
        if let Ok(stick) = q_stick.get(child) {
            commands
                .entity(stick)
                .insert(SwingStickAnimation::default());
        }
    }
}
