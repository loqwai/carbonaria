use crate::components::{ActiveAmmo, AmmoType, Player};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn next_ammo(current: AmmoType) -> AmmoType {
    match current {
        AmmoType::Normal => AmmoType::Poison,
        AmmoType::Poison => AmmoType::RageQuit,
        AmmoType::RageQuit => AmmoType::Reverser,
        AmmoType::Reverser => AmmoType::Normal,
    }
}

fn previous_ammo(current: AmmoType) -> AmmoType {
    match current {
        AmmoType::Normal => AmmoType::Reverser,
        AmmoType::Poison => AmmoType::Normal,
        AmmoType::RageQuit => AmmoType::Poison,
        AmmoType::Reverser => AmmoType::RageQuit,
    }
}

pub fn on_f_key_switch_ammo(
    players: Query<&Children, With<Player>>,
    mut guns: Query<&mut ActiveAmmo>,
    mut keyboard_input: EventReader<KeyboardInput>,
) {
    for event in keyboard_input.iter() {
        match event.state {
            bevy::input::ButtonState::Pressed => match event.key_code {
                Some(KeyCode::F) => {
                    players.iter().for_each(|player_children| {
                        player_children.iter().for_each(|&child| {
                            if let Ok(mut active_weapon) = guns.get_mut(child) {
                                active_weapon.0 = next_ammo(active_weapon.0);
                                return;
                            }
                        });
                    });
                }
                Some(KeyCode::V) => {
                    players.iter().for_each(|player_children| {
                        player_children.iter().for_each(|&child| {
                            if let Ok(mut active_weapon) = guns.get_mut(child) {
                                active_weapon.0 = previous_ammo(active_weapon.0);
                                return;
                            }
                        });
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
