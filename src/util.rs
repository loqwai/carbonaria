use std::time::Instant;

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

type Position = (i16, i16);
/// ViewArea is defined by the upper left and lower right vertices
type ViewArea = (Position, Position);

pub fn is_inside_of(view_area: &ViewArea, position: &Position) -> bool {
    let (x, y) = position;
    let ((x1, y1), (x2, y2)) = view_area;

    return (x1 <= x) && (x <= x2) && (y1 <= y) && (y <= y2);
}
pub fn look_at_target(looker: Vec3, target: Vec3) -> (Quat, Vec3) {
    let diff = looker - target;
    let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
    let rotation = Quat::from_axis_angle(Vec3::Z, angle);
    return (rotation, diff.normalize()*-1.0);
}
