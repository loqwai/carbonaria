use std::collections::HashSet;

use bevy::prelude::*;
use rand::{rngs::SmallRng, seq::IteratorRandom};

use crate::{
    bundles::WallBundle,
    components::{Room, WallType},
};

type Position = (i16, i16);

pub fn spawn_next_tile_for_rooms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rooms_query: Query<&mut Room>,
    mut rng: ResMut<SmallRng>,
) {
    rooms_query.for_each_mut(|room| {
        spawn_next_tile_for_room(&mut commands, &asset_server, rng.as_mut(), room)
    });
}

fn spawn_next_tile_for_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    rng: &mut SmallRng,
    mut room: Mut<Room>,
) {
    if room.complete {
        return;
    }
    // print_map(map, "begin");
    room.add_missing_tiles();
    // print_map(map, "add_missing_tiles");
    room.update_options();
    // print_map(map, "update_options");
    room.update_complete();

    match room.options_tile_with_least_entropy() {
        None => return,
        Some((&pos, tile)) => {
            let options = tile.as_options();

            let wall_type = random_wall_type(rng, &options);
            tile.to_wall_type(wall_type);

            spawn_tile(commands, asset_server, &pos, wall_type);
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: &Position,
    wall_type: WallType,
) {
    let wall = commands
        .spawn_bundle(WallBundle::new(&asset_server, &wall_type, *position))
        .id();

    for shape in wall_type.collision_shapes() {
        let child = commands.spawn().insert(shape).id();
        commands.entity(wall).push_children(&[child]);
    }
}

/// random_wall_type will select a random wall type from the set. If the set is empty, it will return
/// WallType::Empty
fn random_wall_type(rng: &mut SmallRng, wall_types: &HashSet<WallType>) -> WallType {
    if wall_types.is_empty() {
        return WallType::Empty;
    }
    wall_types.iter().choose(rng).unwrap().clone()
}
