use bevy::prelude::*;

use crate::components::{SpinMe, Tick};

pub fn spin_spin_me(mut query: Query<(&mut Transform, &SpinMe)>, ticker: Res<Tick>) {
    let tick = ticker.0 as f32;

    for (mut transform, spin_me) in query.iter_mut() {
        let amount = tick * spin_me.0;
        transform.rotation = Quat::from_rotation_z(amount.to_radians());
    }
}
