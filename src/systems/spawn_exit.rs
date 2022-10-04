use bevy::prelude::*;
use rand::rngs::SmallRng;

use crate::bundles::ExitBundle;
use crate::resources::Config;
use crate::util::random_position;

pub fn spawn_exit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
) {
    let position = random_position(&config, &mut rng);

    commands.spawn_bundle(ExitBundle::new(&asset_server, position));
}
