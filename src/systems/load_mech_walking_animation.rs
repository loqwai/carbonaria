use bevy::prelude::*;

use crate::resources::MechWalkingAnimation;

pub fn load_mech_walking_animation(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MechWalkingAnimation(
        asset_server.load("models/units/mech.gltf#Animation0"),
    ))
}
