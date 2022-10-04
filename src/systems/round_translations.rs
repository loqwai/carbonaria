use bevy::prelude::*;

pub fn round_translations(mut transforms: Query<&mut Transform>) {
    transforms.for_each_mut(|mut transform| {
        transform.translation = transform.translation.floor();
    })
}
