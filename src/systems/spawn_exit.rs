use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::bundles::ExitBundle;
use crate::resources::Config;

pub fn spawn_exit(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    let max: f32 = config.dimensions.into();
    let min: f32 = -max;
    let tile_size: f32 = config.tile_size.into();

    let mut small_rng = SmallRng::from_entropy();
    let x: f32 = tile_size * small_rng.gen_range(min..max);
    let y: f32 = tile_size * small_rng.gen_range(min..max);

    commands.spawn_bundle(ExitBundle::new(&asset_server, Vec3::new(x, y, 0.0)));
}
