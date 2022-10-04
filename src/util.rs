use std::time::Instant;

use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::Rng;

use crate::resources::Config;

#[allow(dead_code)]
pub fn profile<F>(label: &str, f: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    println!("{}: {:?}", label, start.elapsed());
}

pub fn random_position(config: &Res<Config>, rng: &mut ResMut<SmallRng>) -> Vec3 {
    let max: f32 = (config.dimensions / 2).into();
    let min: f32 = -max;
    let tile_size: f32 = config.tile_size.into();

    let x: f32 = tile_size * rng.gen_range(min..max);
    let y: f32 = tile_size * rng.gen_range(min..max);

    Vec3::new(x, y, 0.0)
}
