use std::collections::{hash_map::Iter, HashMap, HashSet};

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
pub struct Room(pub HashMap<Position, Tile>);

impl Room {
    pub fn new() -> Room {
        Room(HashMap::from([((0, 0), Tile::WallType(WallType::Empty))]))
    }

    pub fn iter(&self) -> Iter<Position, Tile> {
        self.0.iter()
    }

    pub fn open_ports(&self) -> HashSet<Port> {
        self.iter()
            .map(ports_for_tile)
            .collect::<Vec<HashSet<Port>>>()
            .iter()
            .flatten()
            .filter(|&port| !self.0.contains_key(&port.position))
            .cloned()
            .collect()
    }
}

type Position = (i16, i16);

fn ports_for_tile((position, tile): (&Position, &Tile)) -> HashSet<Port> {
    match tile {
        Tile::Options(_) => HashSet::new(),
        Tile::WallType(wall_type) => ports_for_wall_type(position, &wall_type)
            .iter()
            .cloned()
            .collect(),
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

fn shift_position((x, y): &Position, (dx, dy): (i16, i16)) -> Position {
    (x + dx, y + dy)
}
