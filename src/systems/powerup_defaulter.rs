use bevy::prelude::*;

pub fn powerup_defaulter<T: Component + Default>(
    mut powerup_target: Query<&mut T>,
) {
    powerup_target.for_each_mut(|mut target| {
        *target = T::default();
    });
}
