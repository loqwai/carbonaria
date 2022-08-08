use std::{cmp::Ordering, collections::HashSet};

use bevy::prelude::*;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::{
    bundles::WallBundle,
    components::{Room, Tile, WallType},
};

type Position = (i16, i16);

pub fn spawn_next_tile_for_rooms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rooms_query: Query<&mut Room>,
) {
    rooms_query.for_each_mut(|room| spawn_next_tile_for_room(&mut commands, &asset_server, room));
}

fn spawn_next_tile_for_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut room: Mut<Room>,
) {
    // print_map(map, "begin");
    room.add_missing_tiles();
    // print_map(map, "add_missing_tiles");
    room.update_options();
    // print_map(map, "update_options");

    let mut rng = SmallRng::from_entropy();
    // let room_clone = room.clone();

    let (&pos, tile) = room
        .iter_mut()
        .filter(|(_, t)| t.is_options())
        .min_by(entropy)
        .unwrap();

    let options = tile.as_options();

    let wall_type = random_wall_type(&mut rng, &options);
    room.tiles.insert(pos, Tile::WallType(wall_type));
    // tile.to_wall_type(wall_type);

    spawn_tile(commands, asset_server, &pos, wall_type);
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

fn entropy((_, t1): &(&Position, &mut Tile), (_, t2): &(&Position, &mut Tile)) -> Ordering {
    match (t1, t2) {
        (Tile::WallType(_), Tile::WallType(_)) => Ordering::Equal,
        (Tile::WallType(_), Tile::Options(_)) => Ordering::Less,
        (Tile::Options(_), Tile::WallType(_)) => Ordering::Greater,
        (Tile::Options(o1), Tile::Options(o2)) => o1.len().cmp(&o2.len()),
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
