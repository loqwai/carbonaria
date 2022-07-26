use bevy::{
    core::Time,
    prelude::{AssetServer, Commands, Res, ResMut},
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::{bundles::MobBundle, resources::MobSpawnTimer};

pub fn spawn_mobs(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MobSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut small_rng = SmallRng::from_entropy();
        let x: f32 = small_rng.gen_range(-64.0..64.0);
        let y: f32 = small_rng.gen_range(-64.0..64.0);
        commands.spawn_bundle(MobBundle::new(asset_server, (x, y)));
    }
}
