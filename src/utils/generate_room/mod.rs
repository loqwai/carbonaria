use core::panic;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

use crate::bundles::WallType;

const DIMENSIONS: i16 = 16;

#[derive(Clone, Debug)]
enum Tile {
    WallType(WallType),
    Options(HashSet<WallType>),
}

type Position = (i16, i16);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PortType {
    Empty,
    EmptyRequired,
    Wall,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Port {
    position: Position,
    port_type: PortType,
}

pub fn generate_room() -> HashMap<Position, WallType> {
    let end: i16 = DIMENSIONS / 2;
    let begin: i16 = end * -1;

    let mut rng = SmallRng::from_entropy();
    let x: i16 = rng.gen_range(begin..=end);
    let y: i16 = rng.gen_range(begin..=end);

    let mut map: HashMap<Position, Tile> = HashMap::new();

    map.insert(
        (x, y),
        Tile::WallType(random_wall_type(&mut rng, &all_wall_types())),
    );

    fill(&mut rng, &mut map);

    to_result(map)
}

fn _print_map(map: &HashMap<Position, Tile>, label: &str) {
    println!("");
    println!("=========={}==========", label);
    for (pos, tile) in map.iter() {
        println!("{:?} => {:?}", pos, tile);
    }
    println!("=========={}==========", label);
}

fn fill(rng: &mut SmallRng, map: &mut HashMap<Position, Tile>) {
    loop {
        // print_map(map, "begin");
        add_missing_tiles(map);
        // print_map(map, "add_missing_tiles");
        update_options(map);
        // print_map(map, "update_options");

        let min = map
            .iter()
            .filter(|(_, t)| is_options(t.clone()))
            .min_by(entropy);

        match min {
            None => return,
            Some((pos, tile)) => match tile {
                Tile::WallType(_) => {
                    panic!("Should not have received a walltype here")
                }
                Tile::Options(wall_types) => {
                    map.insert(*pos, Tile::WallType(random_wall_type(rng, &wall_types)));
                }
            },
        }
    }
}

fn is_options(v: &Tile) -> bool {
    match v {
        Tile::WallType(_) => false,
        Tile::Options(_) => true,
    }
}

fn entropy((_, t1): &(&Position, &Tile), (_, t2): &(&Position, &Tile)) -> Ordering {
    match (t1, t2) {
        (Tile::WallType(_), Tile::WallType(_)) => Ordering::Equal,
        (Tile::WallType(_), Tile::Options(_)) => Ordering::Less,
        (Tile::Options(_), Tile::WallType(_)) => Ordering::Greater,
        (Tile::Options(o1), Tile::Options(o2)) => o1.len().cmp(&o2.len()),
    }
}

fn add_missing_tiles(map: &mut HashMap<Position, Tile>) {
    for port in open_ports(map) {
        let (x, y) = port.position;

        if out_of_range(x) || out_of_range(y) {
            continue;
        }

        map.insert(port.position, Tile::Options(all_wall_types()));
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

fn update_options(map: &mut HashMap<Position, Tile>) {
    for (pos, tile) in map.clone().iter() {
        match tile {
            Tile::WallType(_) => continue,
            Tile::Options(options) => {
                let filtered_options: HashSet<WallType> = options
                    .iter()
                    .filter(|option| is_valid_wall_type_for_position(map, pos, option))
                    .cloned()
                    .collect();

                map.insert(*pos, Tile::Options(filtered_options));
            }
        }
    }
}

fn is_valid_wall_type_for_position(
    map: &HashMap<Position, Tile>,
    position: &Position,
    wall_type: &WallType,
) -> bool {
    for port in ports_for_wall_type(position, wall_type) {
        match map.get(&port.position) {
            None => continue,
            Some(neighbor) => match neighbor {
                Tile::Options(_) => continue,
                Tile::WallType(neighbor_wall_type) => {
                    let neighbors_ports = ports_for_wall_type(&port.position, &neighbor_wall_type);

                    match neighbors_ports
                        .iter()
                        .find(|&neighbors_port| neighbors_port.position == *position)
                    {
                        None => return false,
                        Some(neighbors_port) => {
                            if !is_valid_connection(&neighbors_port.port_type, &port.port_type) {
                                return false;
                            }
                        }
                    }
                }
            },
        }
    }

    true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_map() {
        let map: HashMap<Position, Tile> = HashMap::new();
        let is_valid = is_valid_wall_type_for_position(&map, &(0, 0), &WallType::Vertical);

        assert_eq!(true, is_valid);
    }

    #[test]
    fn test_horizontal_over_vertical() {
        let mut map: HashMap<Position, Tile> = HashMap::new();
        map.insert((0, 0), Tile::WallType(WallType::Vertical));

        let is_valid = is_valid_wall_type_for_position(&map, &(0, 1), &WallType::Horizontal);

        assert_eq!(false, is_valid);
    }

    #[test]
    fn test_horizontal_over_empty() {
        let mut map: HashMap<Position, Tile> = HashMap::new();
        map.insert((0, 0), Tile::WallType(WallType::Empty));

        let is_valid = is_valid_wall_type_for_position(&map, &(0, 1), &WallType::Horizontal);

        assert_eq!(true, is_valid);
    }

    #[test]
    fn test_horizontal_over_horizontal() {
        let mut map: HashMap<Position, Tile> = HashMap::new();
        map.insert((0, 0), Tile::WallType(WallType::Horizontal));

        let is_valid = is_valid_wall_type_for_position(&map, &(0, 1), &WallType::Horizontal);

        assert_eq!(false, is_valid);
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

fn to_result(input: HashMap<Position, Tile>) -> HashMap<Position, WallType> {
    let mut result: HashMap<Position, WallType> = HashMap::new();

    for (&k, v) in input.iter() {
        if let Tile::WallType(wall_type) = v {
            result.insert(k, *wall_type);
        }
    }

    result
}
