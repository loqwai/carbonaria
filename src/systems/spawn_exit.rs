use bevy::prelude::*;

use crate::bundles::ExitBundle;
use crate::components::Exit;
use crate::resources::{Config, SmallRng};
use crate::util::random_position;

pub fn spawn_exit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
    existing_exits: Query<Entity, With<Exit>>,
) {
    if !existing_exits.is_empty() {
        return;
    }
    let position = random_position(&config, &mut rng);

    commands.spawn(ExitBundle::new(&asset_server, position));
}
