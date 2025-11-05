use crate::{
    components::{LaserGun, Player},
    events::ShootEvent,
};
use bevy::{input::ButtonState, prelude::*};

pub fn on_left_click_shoot(
    players: Query<&Children, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut shoot_events: EventWriter<ShootEvent>,
    guns: Query<Entity, With<LaserGun>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for player_children in players.iter() {
            for &child in player_children.iter() {
                let Ok(gun) = guns.get(child) else { continue };

                shoot_events.send(ShootEvent { gun });
            }
        }
    }
}
