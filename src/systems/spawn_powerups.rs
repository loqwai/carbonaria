use crate::components::{Tick, Speed, Team, Health};
use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{bundles::ChestBundle};
use bevy::prelude::*;
use rand::seq::SliceRandom;

lazy_static! {
    static ref POWERUPS: Vec<Powerup> = vec![
        Powerup::Speed(Speed::fast()),
        Powerup::Speed(Speed::fast()),
        Powerup::Speed(Speed::fast()),
        Powerup::Speed(Speed::fast()),
        Powerup::Speed(Speed::fast()),
        Powerup::Speed(Speed::slow()),
        Powerup::Team(Team(0)),
        Powerup::Team(Team(1)),
        Powerup::Health(Health(10)),
    ];
}

#[derive(Clone)]
enum Powerup {
    Speed(Speed),
    Team(Team),
    Health(Health),
}

pub fn spawn_powerups(
    mut commands: Commands,
    ticker: Res<Tick>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
) {
    let ticks = ticker.0;
    if ticks % config.powerup_spawn_interval != 0 {
        return;
    }

    let powerup_entity = match POWERUPS.choose(&mut rng.0).unwrap().clone() {
        Powerup::Speed(s) => commands.spawn(s).id(),
        Powerup::Team(t) => commands.spawn(t).id(),
        Powerup::Health(h) => commands.spawn(h).id(),
    };
    let position = random_position(&config, &mut rng);
    commands.spawn(ChestBundle::new(
        &asset_server,
        position,
        "fast",
        powerup_entity,
    ));

}

