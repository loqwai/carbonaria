use crate::{
    components::{LaserGun, Player},
    events::ShootEvent,
};
use bevy::{input::mouse::MouseButtonInput, prelude::*};

pub fn on_left_click_shoot(
    players: Query<&Children, With<Player>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut shoot_events: EventWriter<ShootEvent>,
    guns: Query<Entity, With<LaserGun>>,
) {
    for _event in mouse_button_events.iter() {
        players.iter().for_each(|player_children| {
            player_children.iter().for_each(|&child| {
                let Ok(gun) = guns.get(child) else { return; };

                shoot_events.send(ShootEvent { gun });
            });
        });
    }
}
