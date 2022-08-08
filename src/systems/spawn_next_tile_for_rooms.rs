use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use bevy::prelude::*;
use heron::CollisionShape;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::{
    bundles::WallBundle,
    components::{Room, Tile, WallType},
};

const DIMENSIONS: i16 = 24;

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
    add_missing_tiles(&mut room);
    // print_map(map, "add_missing_tiles");
    update_options(&mut room);
    // print_map(map, "update_options");

    let mut rng = SmallRng::from_entropy();
    let room_clone = room.clone();
    let min = room_clone
        .iter()
        .filter(|(_, t)| t.is_options())
        .min_by(entropy);

    match min {
        None => return,
        Some((pos, tile)) => match tile {
            Tile::WallType(_) => {
                panic!("Should not have received a walltype here")
            }
            Tile::Options(wall_types) => {
                let wall_type = random_wall_type(&mut rng, &wall_types);
                room.0.insert(*pos, Tile::WallType(wall_type));

                spawn_tile(commands, asset_server, pos, wall_type);
            }
        },
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

    for shape in collision_shapes_for_wall_type(&wall_type) {
        let child = commands.spawn().insert(shape).id();
        commands.entity(wall).push_children(&[child]);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Port {
    position: Position,
    port_type: PortType,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PortType {
    Empty,
    EmptyRequired,
    Wall,
}

fn add_missing_tiles(room: &mut Room) {
    for port in open_ports(&room.0) {
        let (x, y) = port.position;

        if out_of_range(x) || out_of_range(y) {
            continue;
        }

        room.0
            .insert(port.position, Tile::Options(all_wall_types()));
    }
}

fn update_options(room: &mut Room) {
    for (pos, tile) in room.clone().iter() {
        match tile {
            Tile::WallType(_) => continue,
            Tile::Options(options) => {
                let filtered_options: HashSet<WallType> = options
                    .iter()
                    .filter(|option| is_valid_wall_type_for_position(&room.0, pos, option))
                    .cloned()
                    .collect();

                room.0.insert(*pos, Tile::Options(filtered_options));
            }
        }
    }
}

fn out_of_range(n: i16) -> bool {
    let max = DIMENSIONS / 2;
    let min = -max;

    !(min..=max).contains(&n)
}

fn open_ports(map: &HashMap<Position, Tile>) -> HashSet<Port> {
    map.iter()
        .map(ports_for_tile)
        .collect::<Vec<HashSet<Port>>>()
        .iter()
        .flatten()
        .filter(|&port| !map.contains_key(&port.position))
        .cloned()
        .collect()
}

fn entropy((_, t1): &(&Position, &Tile), (_, t2): &(&Position, &Tile)) -> Ordering {
    match (t1, t2) {
        (Tile::WallType(_), Tile::WallType(_)) => Ordering::Equal,
        (Tile::WallType(_), Tile::Options(_)) => Ordering::Less,
        (Tile::Options(_), Tile::WallType(_)) => Ordering::Greater,
        (Tile::Options(o1), Tile::Options(o2)) => o1.len().cmp(&o2.len()),
    }
}

fn ports_for_wall_type(position: &Position, wall_type: &WallType) -> HashSet<Port> {
    match wall_type {
        WallType::Empty => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::Empty,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::Empty,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::Empty,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::Empty,
            },
        ]),
        WallType::Vertical => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::EmptyRequired,
            },
        ]),
        WallType::Horizontal => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::Wall,
            },
        ]),
        WallType::TopLeftCorner => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::EmptyRequired,
            },
        ]),
        WallType::TopRightCorner => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::Wall,
            },
        ]),
        WallType::BottomRightCorner => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::Wall,
            },
        ]),
        WallType::BottomLeftCorner => HashSet::from([
            Port {
                position: shift_position(position, (0, 1)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (1, 0)),
                port_type: PortType::Wall,
            },
            Port {
                position: shift_position(position, (0, -1)),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: shift_position(position, (-1, 0)),
                port_type: PortType::EmptyRequired,
            },
        ]),
    }
}

fn ports_for_tile((position, tile): (&Position, &Tile)) -> HashSet<Port> {
    match tile {
        Tile::Options(_) => HashSet::new(),
        Tile::WallType(wall_type) => ports_for_wall_type(position, &wall_type)
            .iter()
            .cloned()
            .collect(),
    }
}

fn shift_position((x, y): &Position, (dx, dy): (i16, i16)) -> Position {
    (x + dx, y + dy)
}

fn all_wall_types() -> HashSet<WallType> {
    HashSet::from([
        WallType::Empty,
        WallType::Horizontal,
        WallType::Vertical,
        WallType::TopLeftCorner,
        WallType::TopRightCorner,
        WallType::BottomRightCorner,
        WallType::BottomLeftCorner,
    ])
}

/// random_wall_type will select a random wall type from the set. If the set is empty, it will return
/// WallType::Empty
fn random_wall_type(rng: &mut SmallRng, wall_types: &HashSet<WallType>) -> WallType {
    if wall_types.is_empty() {
        return WallType::Empty;
    }
    wall_types.iter().choose(rng).unwrap().clone()
}

fn is_valid_wall_type_for_position(
    map: &HashMap<Position, Tile>,
    position: &Position,
    wall_type: &WallType,
) -> bool {
    for port in ports_for_wall_type(position, wall_type) {
        match map.get(&port.position) {
            None => continue,
            Some(neighbor) => {
                let wall_types: Vec<WallType> = neighbor.into();

                if none_compatible(position, &port.port_type, &port.position, &wall_types) {
                    return false;
                }
            }
        }
    }

    true
}

fn none_compatible(
    position: &(i16, i16),
    port_type: &PortType,
    neighbor_position: &Position,
    neighbor_wall_types: &Vec<WallType>,
) -> bool {
    !neighbor_wall_types.iter().any(|neighbor_wall_type| {
        compatible_neighbor(position, port_type, neighbor_position, neighbor_wall_type)
    })
}

fn compatible_neighbor(
    my_position: &Position,
    my_port_type: &PortType,
    neighbor_position: &Position,
    neighbor_wall_type: &WallType,
) -> bool {
    match find_neighbors_port(my_position, neighbor_position, neighbor_wall_type) {
        None => true,
        Some(neighbors_port) => is_valid_connection(&neighbors_port.port_type, my_port_type),
    }
}

fn is_valid_connection(p1: &PortType, p2: &PortType) -> bool {
    match (p1, p2) {
        (PortType::Empty, PortType::Empty) => true,
        (PortType::Empty, PortType::EmptyRequired) => true,
        (PortType::Empty, PortType::Wall) => false,

        (PortType::EmptyRequired, PortType::Empty) => true,
        (PortType::EmptyRequired, _) => false,

        (PortType::Wall, PortType::Wall) => true,
        (PortType::Wall, _) => false,
    }
}

fn find_neighbors_port(
    my_position: &Position,
    neighbor_position: &Position,
    neighbor_wall_type: &WallType,
) -> Option<Port> {
    for port in ports_for_wall_type(&neighbor_position, &neighbor_wall_type) {
        if port.position == *my_position {
            return Some(port);
        }
    }

    return None;
}

fn collision_shapes_for_wall_type(wall_type: &WallType) -> Vec<CollisionShape> {
    // Note: These aren't adjusted for rotation since our transform should do that for us.

    match wall_type {
        WallType::Empty => Vec::new(),
        WallType::Vertical => straight_piece(),
        WallType::Horizontal => straight_piece(),
        WallType::TopLeftCorner => corner_piece(),
        WallType::TopRightCorner => corner_piece(),
        WallType::BottomRightCorner => corner_piece(),
        WallType::BottomLeftCorner => corner_piece(),
    }
}

fn straight_piece() -> Vec<CollisionShape> {
    vec![cuboid(32.0, 16.0)]
}

fn corner_piece() -> Vec<CollisionShape> {
    vec![
        convex(vec![
            (-32.0, 16.0),
            (-16.0, 16.0),
            (-16.0, -16.0),
            (-32.0, -16.0),
        ]),
        convex(vec![
            (-16.0, 32.0),
            (16.0, 32.0),
            (16.0, -16.0),
            (-16.0, -16.0),
        ]),
    ]
}

fn convex(points: Vec<(f32, f32)>) -> CollisionShape {
    let points: Vec<Vec3> = points.iter().map(|(x, y)| Vec3::new(*x, *y, 0.0)).collect();

    CollisionShape::ConvexHull {
        points,
        border_radius: None,
    }
}

fn cuboid(half_width: f32, half_height: f32) -> CollisionShape {
    CollisionShape::Cuboid {
        half_extends: Vec3::new(half_width, half_height, 0.0),
        border_radius: None,
    }
}
