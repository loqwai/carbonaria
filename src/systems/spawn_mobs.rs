use bevy::prelude::{AnimationClip, AssetServer, Assets, BuildChildren, Commands, Res, ResMut};

use bevy::time::Time;
use rand::rngs::SmallRng;

use crate::resources::Config;
use crate::util::random_position;
use crate::{
    bundles::{MobBundle, StickBundle},
    resources::MobSpawnTimer,
};

pub fn spawn_mobs(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MobSpawnTimer>,
    asset_server: Res<AssetServer>,
    animations: ResMut<Assets<AnimationClip>>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let position = random_position(&config, &mut rng);

        let mob = commands
            .spawn_bundle(MobBundle::new(&asset_server, position))
            .id();
        let stick = commands
            .spawn_bundle(StickBundle::new(&asset_server, animations))
            .id();
        commands.entity(mob).push_children(&[stick]);
    }
}
