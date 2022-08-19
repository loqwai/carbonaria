use std::collections::HashSet;

use bevy::prelude::*;
use rand::{rngs::SmallRng, seq::IteratorRandom};
use std::time::Instant;

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
    let start = Instant::now();

    loop {
        if room.is_complete() {
            println!("elapsed: {:?}, {}", start.elapsed(), room.known_tiles.len());
            panic!("panicking!");
            // return;
        }

        remove_impossible_options(&mut room);

        match room.options_with_least_entropy() {
            None => return,
            Some((pos, options)) => {
                let wall_type = random_wall_type(rng, &options);
                room.known_tiles.insert(pos, wall_type);
                room.options_tiles.remove(&pos);
                spawn_tile(commands, asset_server, &pos, wall_type);

                for neighbor_position in neighbor_positions_for_position(&pos) {
                    if room.out_of_bounds(&neighbor_position) || room.occupied_positions.contains(&neighbor_position) {
                        continue
                    }

                    room.options_tiles.insert(neighbor_position, WallType::all());
                    room.occupied_positions.insert(neighbor_position);
                }
            }
        }
    }
}

/// update_options mutates the room's tiles to remove all
/// options that are not allowed due to port mismatches.
pub fn remove_impossible_options(room: &mut Room) {
    for (pos, options) in room.options_tiles.clone().iter_mut() {
        options.retain(|option| room.is_valid_wall_type_for_position(pos, &option));
        room.options_tiles.insert(*pos, options.clone());
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

fn neighbor_positions_for_position(position: &Position) -> HashSet<Position> {
    HashSet::from([
        shift_position(position, (0, 1)),
        shift_position(position, (1, 0)),
        shift_position(position, (0, -1)),
        shift_position(position, (-1, 0)),
    ])
}

fn shift_position((x, y): &Position, (dx, dy): (i16, i16)) -> Position {
    (x + dx, y + dy)
}
