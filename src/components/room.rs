use std::{
    cmp::Ordering,
    collections::{
        hash_map::{Iter, IterMut},
        HashMap, HashSet,
    },
};

use bevy::prelude::*;

use super::wall_type::{Port, PortType, WallType};

#[derive(Clone, Debug)]
pub enum Tile {
    WallType(WallType),
    Options(HashSet<WallType>),
}

impl Tile {
    pub fn is_options(&self) -> bool {
        match self {
            Tile::WallType(_) => false,
            Tile::Options(_) => true,
        }
    }

    pub fn is_wall_type(&self) -> bool {
        match self {
            Tile::WallType(_) => true,
            Tile::Options(_) => false,
        }
    }

    pub fn as_options(&self) -> &HashSet<WallType> {
        match self {
            Tile::WallType(_) => panic!("Called as_options on a Tile that was a WallType"),
            Tile::Options(options) => options,
        }
    }

    pub fn to_wall_type(&mut self, wall_type: WallType) {
        *self = Tile::WallType(wall_type)
    }
}

impl Into<Vec<WallType>> for &Tile {
    fn into(self) -> Vec<WallType> {
        match self {
            Tile::WallType(wall_type) => vec![wall_type.clone()],
            Tile::Options(options) => options.iter().cloned().collect(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Room {
    pub dimensions: i16,
    pub complete: bool,
    pub tiles: HashMap<Position, Tile>,
}

impl Room {
    pub fn new(dimensions: i16) -> Room {
        Room {
            dimensions,
            complete: false,
            tiles: HashMap::from([((0, 0), Tile::WallType(WallType::Empty))]),
        }
    }

    pub fn iter(&self) -> Iter<Position, Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Position, Tile> {
        self.tiles.iter_mut()
    }

    /// add_missing_tiles iterates over all the confirmed tiles
    /// and ensures that their direct neighbors all have tiles
    pub fn add_missing_tiles(&mut self) {
        for (x, y) in self.open_port_positions() {
            if self.out_of_range(x) || self.out_of_range(y) {
                continue;
            }

            self.tiles.insert((x, y), Tile::Options(WallType::all()));
        }
    }

    /// update_options mutates the room's tiles to remove all
    /// options that are not allowed due ot port mismatches.
    pub fn update_options(&mut self) {
        for (pos, tile) in self.clone().iter_mut() {
            match tile {
                Tile::WallType(_) => continue,
                Tile::Options(options) => {
                    options.retain(|option| self.is_valid_wall_type_for_position(pos, &option));
                    self.tiles.insert(*pos, Tile::Options(options.clone()));
                }
            }
        }
    }

    /// update_complete checks to see if there are any open port left. If not,
    /// then it updates room.complete to be true.
    pub fn update_complete(&mut self) {
        if self.iter().all(|(_, t)| t.is_wall_type()) {
            self.complete = true
        }
    }

    pub fn options_tile_with_least_entropy(&mut self) -> Option<(&Position, &mut Tile)> {
        self.iter_mut()
            .filter(|(_, t)| t.is_options())
            .min_by(entropy)
    }

    fn is_valid_wall_type_for_position(&self, position: &Position, wall_type: &WallType) -> bool {
        for port in wall_type.ports(position) {
            match self.tiles.get(&port.position) {
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

    fn open_port_positions(&self) -> HashSet<Position> {
        self.iter()
            .map(tile_neighbors)
            .collect::<Vec<HashSet<Position>>>()
            .iter()
            .flatten()
            .filter(|&p| !self.tiles.contains_key(&p))
            .cloned()
            .collect()
    }

    fn out_of_range(&self, n: i16) -> bool {
        let max = self.dimensions / 2;
        let min = -max;

        !(min..=max).contains(&n)
    }
}

type Position = (i16, i16);

fn tile_neighbors((position, tile): (&Position, &Tile)) -> HashSet<Position> {
    match tile {
        Tile::Options(_) => HashSet::new(),
        Tile::WallType(_) => neighbor_positions_for_position(position),
    }
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
    for port in neighbor_wall_type.ports(&neighbor_position) {
        if port.position == *my_position {
            return Some(port);
        }
    }

    return None;
}

fn entropy((_, t1): &(&Position, &mut Tile), (_, t2): &(&Position, &mut Tile)) -> Ordering {
    match (t1, t2) {
        (Tile::WallType(_), Tile::WallType(_)) => Ordering::Equal,
        (Tile::WallType(_), Tile::Options(_)) => Ordering::Less,
        (Tile::Options(_), Tile::WallType(_)) => Ordering::Greater,
        (Tile::Options(o1), Tile::Options(o2)) => o1.len().cmp(&o2.len()),
    }
}
