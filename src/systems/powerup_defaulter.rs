use bevy::prelude::*;

pub fn powerup_defaulter<T: Component + Default>(mut powerup_target: Query<&mut T>) {
    for mut target in powerup_target.iter_mut() {
        *target = T::default();
    }
}
