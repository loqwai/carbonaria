use std::collections::HashSet;

use bevy::{prelude::*, render::primitives::Frustum};
use rand::{rngs::SmallRng, seq::IteratorRandom};

use crate::{
    bundles::WallBundle,
    components::{Room, WallType},
};

type Position = (i16, i16);
/// ViewArea is defined by the upper left and lower right vertices
type ViewArea = (Position, Position);

pub fn spawn_next_tile_for_rooms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rooms_query: Query<&mut Room>,
    mut rng: ResMut<SmallRng>,
    frustrums: Query<&Frustum, With<Camera>>,
) {
    let view_area = view_area_from_frustrum(frustrums.get_single().unwrap());

    rooms_query.for_each_mut(|room| {
        spawn_next_tile_for_room(&mut commands, &asset_server, rng.as_mut(), room, &view_area)
    });
}

fn spawn_next_tile_for_room(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    rng: &mut SmallRng,
    mut room: Mut<Room>,
    view_area: &ViewArea,
) {
    if room.is_complete() {
        return;
    }

    remove_impossible_options(commands, asset_server, &mut room);

    match room.options_with_least_entropy(view_area) {
        None => return,
        Some((pos, options)) => {
            let wall_type = random_wall_type(rng, &options);

            confirm_wall_type(commands, asset_server, &mut room, pos, wall_type);
        }
    }
}

fn view_area_from_frustrum(frustum: &Frustum) -> ViewArea {
    let tile_size: f32 = 64.0;
    let [left, right, top, bottom, _near, _far] = frustum.planes;

    let x1 = -(left.d() / tile_size).round() as i16;
    let y1 = -(top.d() / tile_size).round() as i16;
    let x2 = (right.d() / tile_size).round() as i16;
    let y2 = (bottom.d() / tile_size).round() as i16;

    ((x1, y1), (x2, y2))
}

fn confirm_wall_type(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    room: &mut Room,
    pos: (i16, i16),
    wall_type: WallType,
) {
    room.known_tiles.insert(pos, wall_type);
    room.options_tiles.remove(&pos);
    spawn_tile(commands, asset_server, &pos, wall_type);
    for neighbor_position in neighbor_positions_for_position(&pos) {
        if room.out_of_bounds(&neighbor_position) || room.is_occupied(&neighbor_position) {
            continue;
        }

        room.options_tiles
            .insert(neighbor_position, WallType::all());
    }
}

/// update_options mutates the room's tiles to remove all
/// options that are not allowed due to port mismatches.
pub fn remove_impossible_options(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    room: &mut Room,
) {
    for (pos, options) in room.options_tiles.clone().iter_mut() {
        options.retain(|option| room.is_valid_wall_type_for_position(pos, &option));
        room.options_tiles.insert(*pos, options.clone());

        if options.len() == 1 {
            confirm_wall_type(
                commands,
                asset_server,
                room,
                *pos,
                *options.iter().next().unwrap(),
            )
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

/// random_wall_type will select a random wall type from the set. If the set is empty, it will return an empty wall type
fn random_wall_type(rng: &mut SmallRng, wall_types: &HashSet<WallType>) -> WallType {
    if wall_types.is_empty() {
        return WallType::empty();
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
