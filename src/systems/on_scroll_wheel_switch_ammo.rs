use crate::components::{ActiveAmmo, AmmoType, Player};
use bevy::{input::mouse::MouseWheel, prelude::*};

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

pub fn on_scroll_wheel_switch_ammo(
    players: Query<&Children, With<Player>>,
    mut guns: Query<&mut ActiveAmmo>,
    mut wheel_events: EventReader<MouseWheel>,
) {
    for event in wheel_events.iter() {
        players.iter().for_each(|player_children| {
            player_children.iter().for_each(|&child| {
                if let Ok(mut active_weapon) = guns.get_mut(child) {
                    if event.y > 0.0 {
                        active_weapon.0 = next_ammo(active_weapon.0);
                        return;
                    }
                    active_weapon.0 = previous_ammo(active_weapon.0);
                }
            });
        });
    }
}
