use bevy::prelude::{AssetServer, BuildChildren, Commands, Res, ResMut};

use bevy::time::Time;

use crate::resources::{Config, SmallRng};
use crate::util::random_position;
use crate::{
    bundles::{MobBundle, LaserGunBundle},
    resources::MobSpawnTimer,
};

pub fn spawn_mobs(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MobSpawnTimer>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut rng: ResMut<SmallRng>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let position = random_position(&config, &mut rng);

        let mob = commands.spawn(MobBundle::new(&asset_server, position)).id();
        let laser_gun = commands
            .spawn(LaserGunBundle::new(&asset_server))
            .id();
        commands.entity(mob).push_children(&[laser_gun]);
    }
}
