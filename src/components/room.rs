use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use bevy::prelude::Component;

use super::wall_type::{Port, PortType, WallType};

type Position = (i16, i16);
/// ViewArea is defined by the upper left and lower right vertices
type ViewArea = (Position, Position);

#[derive(Clone, Component)]
pub struct Room {
    pub dimensions: i16,
    pub known_tiles: HashMap<Position, WallType>,
    pub options_tiles: HashMap<Position, HashSet<WallType>>,
}

impl Room {
    pub fn new(dimensions: i16) -> Room {
        Room {
            dimensions,
            known_tiles: HashMap::from([((0, 0), WallType::empty())]),
            options_tiles: HashMap::from([
                ((0, 1), WallType::all()),
                ((0, -1), WallType::all()),
                ((1, 0), WallType::all()),
                ((-1, 0), WallType::all()),
            ]),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.known_tiles.len() > 1 && self.options_tiles.is_empty()
    }

    pub fn is_occupied(&self, pos: &Position) -> bool {
        self.known_tiles.contains_key(pos) || self.options_tiles.contains_key(pos)
    }

    pub fn options_with_least_entropy(
        &self,
        view_area: &ViewArea,
    ) -> Option<(Position, HashSet<WallType>)> {
        let (position, options) = self
            .options_tiles
            .iter()
            .filter(|(position, _)| {
                let (x, y) = position;
                let ((x1, y1), (x2, y2)) = view_area;

                return (x1 <= x) && (x <= x2) && (y1 <= y) && (y <= y2);
            })
            .min_by(entropy)?;

        return Some((position.clone(), options.clone()));
    }

    /// get_wall_types_for_position will return the known position as a single item HashSet if its known,
    /// if not the position is unknown, it will return the options for that position. If the options aren't
    /// defined, it will return none
    fn get_wall_types_for_position(&self, position: &Position) -> Option<HashSet<WallType>> {
        match self.known_tiles.get(position) {
            Some(wall_type) => Some(HashSet::from([wall_type.clone()])),
            None => self
                .options_tiles
                .get(position)
                .map(|options| options.clone()),
        }
    }

    pub fn is_valid_wall_type_for_position(
        &self,
        position: &Position,
        wall_type: &WallType,
    ) -> bool {
        for port in wall_type.ports(position) {
            match self.get_wall_types_for_position(&port.position) {
                None => continue,
                Some(wall_types) => {
                    if none_compatible(position, &port.port_type, &port.position, &wall_types) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn out_of_bounds(&self, (x, y): &Position) -> bool {
        self.out_of_range(x) || self.out_of_range(y)
    }

    pub fn out_of_range(&self, n: &i16) -> bool {
        let max = self.dimensions / 2;
        let min = -max;

        !(min..=max).contains(n)
    }
}

/// none_compatible returns true if none of the possible wall_types contain a port
/// that could interface with our port.
fn none_compatible(
    position: &(i16, i16),
    port_type: &PortType,
    neighbor_position: &Position,
    neighbor_wall_types: &HashSet<WallType>,
) -> bool {
    !any_compatible(position, port_type, neighbor_position, neighbor_wall_types)
}

/// any_compatible returns true if at least one of the possible wall_types contain
/// a port that could interface with our port.
fn any_compatible(
    position: &(i16, i16),
    port_type: &PortType,
    neighbor_position: &Position,
    neighbor_wall_types: &HashSet<WallType>,
) -> bool {
    neighbor_wall_types.iter().any(|neighbor_wall_type| {
        compatible_neighbor(position, port_type, neighbor_position, neighbor_wall_type)
    })
}

/// compatible_neighbor returns true if the other walltype has a port facing us that
/// can interface with our port.
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

fn entropy(
    (_, o1): &(&Position, &HashSet<WallType>),
    (_, o2): &(&Position, &HashSet<WallType>),
) -> Ordering {
    o1.len().cmp(&o2.len())
}
