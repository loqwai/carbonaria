use std::collections::HashSet;

use bevy::prelude::*;
use rand::{rngs::SmallRng, seq::IteratorRandom};

use crate::{
    bundles::WallBundle,
    components::{Room, Tile, WallType},
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
    add_missing_tiles(&mut room);
    remove_impossible_options(&mut room);
    update_complete(&mut room);

    match room.position_of_options_tile_with_least_entropy() {
        None => return,
        Some(pos) => {
            let tile = room.tiles.get(&pos).unwrap();
            let wall_type = random_wall_type(rng, tile.as_options());

            room.tiles.insert(pos, Tile::WallType(wall_type));
            spawn_tile(commands, asset_server, &pos, wall_type);
        }
    }
}

/// add_missing_tiles iterates over all the confirmed tiles
/// and ensures that their direct neighbors all have tiles
pub fn add_missing_tiles(room: &mut Room) {
    for (x, y) in room.open_port_positions() {
        if room.out_of_range(x) || room.out_of_range(y) {
            continue;
        }

        room.tiles.insert((x, y), Tile::Options(WallType::all()));
    }
}

/// update_options mutates the room's tiles to remove all
/// options that are not allowed due to port mismatches.
pub fn remove_impossible_options(room: &mut Room) {
    for (pos, tile) in room.clone().tiles.iter_mut() {
        match tile {
            Tile::WallType(_) => continue,
            Tile::Options(options) => {
                options.retain(|option| room.is_valid_wall_type_for_position(pos, &option));
                room.tiles.insert(*pos, Tile::Options(options.clone()));
            }
        }
    }
}

/// update_complete checks to see if there are any open port left. If not,
/// then it updates room.complete to be true.
pub fn update_complete(room: &mut Room) {
    if room.tiles.iter().all(|(_, t)| t.is_wall_type()) {
        room.complete = true
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
