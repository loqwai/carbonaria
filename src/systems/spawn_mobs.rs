use bevy::prelude::*;

use crate::components::{Tick, RateOfFire, Math, Speed};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{
    bundles::{LaserGunBundle, MobBundle},
};

pub fn spawn_mobs(
    mut commands: Commands,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ticks = ticker.0;
    if ticks % config.mob_spawn_interval != 0 {
        return;
    }
    let position = random_position(&config, &mut rng);
    let mob = commands.spawn(MobBundle::new(&asset_server, &mut texture_atlases, position, config.scale)).id();
    let laser_gun = commands.spawn(LaserGunBundle::new(60)).id();
    let rate_of_fire_powerup = commands.spawn(Math::add(RateOfFire(0.1))).id();
    let speed_powerup = commands.spawn(Math::add(Speed(1.0))).id();
    let health_powerup = commands.spawn(Math::add(crate::components::Health(10))).id();

    commands.entity(mob).push_children(&[laser_gun, speed_powerup, health_powerup, rate_of_fire_powerup]);
}
