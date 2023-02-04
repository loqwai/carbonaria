use std::{time::Instant, f32::consts::PI};

use crate::resources::SmallRng;
use bevy::prelude::*;
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

pub fn look_at_target(looker: Vec3, target: Vec3) -> (Quat, Vec3) {
    let diff = target - looker;
    let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
    let rotation = Quat::from_axis_angle(Vec3::Z, angle);
    return (rotation, diff.normalize());
}

/// returns the angle of the vector in radians
pub fn vector_angle(direction: Vec3) -> f32 {
    let x = direction.x;
    let y = direction.y;
    
    let mut angle = x.atan2(y);

    if angle < 0.0 {
        angle += PI * 2.0;
    }

    angle
}

pub fn index_for_direction(direction: Vec3, len: usize) -> usize {
    let angle = vector_angle(direction).to_degrees().round() as usize;
    let partition_size = 360 / len;

    (angle / partition_size) % len
}