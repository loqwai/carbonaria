use bevy::prelude::*;

use crate::components::{Tick, RateOfFire, AddPowerup};
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
    let rate_of_fire = commands.spawn(RateOfFire::default() ).id();
    let rate_of_fire_powerup = commands.spawn(AddPowerup(RateOfFire(1))).id();

    commands.entity(mob).push_children(&[laser_gun, rate_of_fire, rate_of_fire_powerup]);
}
