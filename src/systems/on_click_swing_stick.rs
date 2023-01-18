use bevy::prelude::*;

use crate::{
    components::{Player, Stick},
    events::SwingStickEvent,
};

pub fn on_click_swing_stick(
    mouse_input: Res<Input<MouseButton>>,
    q_player: Query<&Children, With<Player>>,
    q_stick: Query<Entity, With<Stick>>,
    mut writer: EventWriter<SwingStickEvent>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    q_player.for_each(|children| {
        for &child in children.iter() {
            if let Ok(stick) = q_stick.get(child) {
                writer.send(SwingStickEvent { stick })
            }
        }
    });
}
