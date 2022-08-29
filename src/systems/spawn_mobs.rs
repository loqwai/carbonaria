use bevy::{
    core::Time,
    prelude::{AnimationClip, AssetServer, Assets, BuildChildren, Commands, Res, ResMut},
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

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
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut small_rng = SmallRng::from_entropy();
        let x: f32 = small_rng.gen_range(-64.0..64.0);
        let y: f32 = small_rng.gen_range(-64.0..64.0);

        let mob = commands
            .spawn_bundle(MobBundle::new(&asset_server, (x, y)))
            .id();
        let stick = commands
            .spawn_bundle(StickBundle::new(&asset_server, animations))
            .id();
        commands.entity(mob).push_children(&[stick]);
    }
}
