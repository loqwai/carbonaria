use std::ops::{MulAssign, AddAssign};

use bevy::prelude::*;

use crate::components::Math;

pub fn powerup_mather<T: Component + MulAssign + AddAssign + Clone>(
    powerups: Query<(&Parent, &Math<T>)>,
    entities: Query<Entity, (With<Parent>, With<Math<T>>)>,
    mut powerup_target: Query<&mut T>,
) {
    let mut entities: Vec<Entity> = entities.iter().collect();
    entities.sort();

    for entity in entities {
        let (parent, powerup) = powerups.get(entity).unwrap();

        if let Ok(mut target) = powerup_target.get_mut(parent.get()) {
            let powerup = powerup.clone();

            if let Some(add) = powerup.add {
                *target += add;
            }

            if let Some(multiply) = powerup.multiply {
                *target *= multiply;
            }
        }
    };
}