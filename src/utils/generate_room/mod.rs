use core::panic;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

use crate::bundles::WallType;

const DIMENSIONS: i16 = 8;

#[derive(Clone, Debug)]
enum WallTypeOrOptions {
    WallType(WallType),
    Options(HashSet<WallType>),
}

#[derive(Clone)]
pub struct Tile {
    value: WallTypeOrOptions,
}

type Position = (i16, i16);

pub fn generate_room() -> HashMap<Position, WallType> {
    let end: i16 = DIMENSIONS / 2;
    let begin: i16 = end * -1;

    let mut rng = SmallRng::from_entropy();
    let x: i16 = rng.gen_range(begin..=end);
    let y: i16 = rng.gen_range(begin..=end);

    let mut map: HashMap<Position, Tile> = HashMap::new();

    map.insert(
        (x, y),
        Tile {
            value: WallTypeOrOptions::WallType(random_wall_type(&mut rng, &all_wall_types())),
        },
    );

    fill(&mut rng, &mut map);

    to_result(map)
}

// fn print_map(map: &HashMap<Position, Tile>, label: &str) {
//     println!("");
//     println!("=========={}==========", label);
//     for (pos, tile) in map.iter() {
//         println!("{:?} => {:?}", pos, tile.value);
//     }
//     println!("=========={}==========", label);
// }

fn fill(rng: &mut SmallRng, map: &mut HashMap<Position, Tile>) {
    loop {
        // print_map(map, "begin");
        add_missing_tiles(map);
        // print_map(map, "add_missing_tiles");
        update_options(map);
        // print_map(map, "update_options");

        let min = map
            .iter()
            .filter(|(_, t)| is_options(t.value.clone()))
            .min_by(entropy);

        match min {
            None => return,
            Some((pos, tile)) => match &tile.value {
                WallTypeOrOptions::WallType(_) => {
                    panic!("Should not have received a walltype here")
                }
                WallTypeOrOptions::Options(wall_types) => {
                    map.insert(
                        *pos,
                        Tile {
                            value: WallTypeOrOptions::WallType(random_wall_type(rng, &wall_types)),
                        },
                    );
                }
            },
        }
    }
}

fn is_options(v: WallTypeOrOptions) -> bool {
    match v {
        WallTypeOrOptions::WallType(_) => false,
        WallTypeOrOptions::Options(_) => true,
    }
}

fn entropy((_, t1): &(&Position, &Tile), (_, t2): &(&Position, &Tile)) -> Ordering {
    match (&t1.value, &t2.value) {
        (WallTypeOrOptions::WallType(_), WallTypeOrOptions::WallType(_)) => Ordering::Equal,
        (WallTypeOrOptions::WallType(_), WallTypeOrOptions::Options(_)) => Ordering::Less,
        (WallTypeOrOptions::Options(_), WallTypeOrOptions::WallType(_)) => Ordering::Greater,
        (WallTypeOrOptions::Options(o1), WallTypeOrOptions::Options(o2)) => o1.len().cmp(&o2.len()),
    }
}

fn add_missing_tiles(map: &mut HashMap<Position, Tile>) {
    for port in open_ports(map) {
        map.insert(
            port,
            Tile {
                value: WallTypeOrOptions::Options(all_wall_types()),
            },
        );
    }
}

fn open_ports(map: &HashMap<Position, Tile>) -> HashSet<Position> {
    map.iter()
        .map(ports_for_tile)
        .collect::<Vec<HashSet<Position>>>()
        .iter()
        .flatten()
        .filter(|&position| !map.contains_key(position))
        .cloned()
        .collect()
}

fn update_options(map: &mut HashMap<Position, Tile>) {
    for (pos, tile) in map.clone().iter() {
        match &tile.value {
            WallTypeOrOptions::WallType(_) => continue,
            WallTypeOrOptions::Options(options) => {
                let filtered_options: HashSet<WallType> = options
                    .iter()
                    .filter(|option| is_valid_wall_type_for_position(map, pos, option))
                    .cloned()
                    .collect();

                map.insert(
                    *pos,
                    Tile {
                        value: WallTypeOrOptions::Options(filtered_options),
                    },
                );
            }
        }
    }
}

fn is_valid_wall_type_for_position(
    map: &HashMap<Position, Tile>,
    position: &Position,
    wall_type: &WallType,
) -> bool {
    let (x, y) = position;

    match position {
        _ if *x > DIMENSIONS => return false,
        _ if *y > DIMENSIONS => return false,
        _ if *x < -DIMENSIONS => return false,
        _ if *y < -DIMENSIONS => return false,
        _ => (),
    }

    for neighbor in ports_for_wall_type(position, wall_type) {
        match map.get(&neighbor) {
            None => continue,
            Some(neighboring_tile) => match neighboring_tile.value {
                WallTypeOrOptions::Options(_) => continue,
                WallTypeOrOptions::WallType(neighbor_wall_type) => {
                    let opposing_ports = ports_for_wall_type(&neighbor, &neighbor_wall_type);

                    if !opposing_ports.contains(position) {
                        return false;
                    }
                }
            },
        }
    }

    true
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
        map.insert(
            (0, 0),
            Tile {
                value: WallTypeOrOptions::WallType(WallType::Vertical),
            },
        );

        let is_valid = is_valid_wall_type_for_position(&map, &(0, 1), &WallType::Horizontal);

        assert_eq!(false, is_valid);
    }
}

fn ports_for_wall_type(position: &Position, wall_type: &WallType) -> HashSet<Position> {
    match wall_type {
        WallType::Empty => HashSet::new(),
        WallType::Vertical => HashSet::from([
            shift_position(position, (0, 1)),
            shift_position(position, (0, -1)),
        ]),
        WallType::Horizontal => HashSet::from([
            shift_position(position, (1, 0)),
            shift_position(position, (-1, 0)),
        ]),
        WallType::TopLeftCorner => HashSet::from([
            shift_position(position, (1, 0)),
            shift_position(position, (0, -1)),
        ]),
        WallType::TopRightCorner => HashSet::from([
            shift_position(position, (-1, 0)),
            shift_position(position, (0, -1)),
        ]),
        WallType::BottomRightCorner => HashSet::from([
            shift_position(position, (-1, 0)),
            shift_position(position, (0, 1)),
        ]),
        WallType::BottomLeftCorner => HashSet::from([
            shift_position(position, (1, 0)),
            shift_position(position, (0, 1)),
        ]),
    }
}

fn ports_for_tile((position, tile): (&Position, &Tile)) -> HashSet<Position> {
    match tile.value {
        WallTypeOrOptions::Options(_) => HashSet::new(),
        WallTypeOrOptions::WallType(wall_type) => ports_for_wall_type(position, &wall_type)
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
        if let WallTypeOrOptions::WallType(wall_type) = v.value {
            result.insert(k, wall_type);
        }
    }

    result
}
