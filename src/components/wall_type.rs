use std::collections::HashSet;

use bevy::prelude::{Component, Vec3};
use heron::CollisionShape;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TileType {
    Empty,
    Straight,
    Corner,
    Tee,
}

impl TileType {
    pub fn all() -> HashSet<TileType> {
        HashSet::from([
            TileType::Empty,
            TileType::Straight,
            TileType::Corner,
            TileType::Tee,
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuarterRotation {
    Zero,
    One,
    Two,
    Three,
}

impl QuarterRotation {
    pub fn all() -> HashSet<QuarterRotation> {
        HashSet::from([
            QuarterRotation::Zero,
            QuarterRotation::One,
            QuarterRotation::Two,
            QuarterRotation::Three,
        ])
    }
}

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq, Hash)]
pub struct WallType {
    pub tile_type: TileType,
    pub quarter_rotations: QuarterRotation,
}

type Position = (i16, i16);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Port {
    pub position: Position,
    pub port_type: PortType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PortType {
    Empty,
    EmptyRequired,
    Wall,
}

impl WallType {
    pub fn empty() -> WallType {
        WallType {
            tile_type: TileType::Empty,
            quarter_rotations: QuarterRotation::Zero,
        }
    }

    /// all returns a hashset of all of the possible walltypes
    pub fn all() -> HashSet<WallType> {
        let mut all: HashSet<WallType> = HashSet::new();

        for tile_type in TileType::all() {
            for quarter_rotations in QuarterRotation::all() {
                all.insert(WallType {
                    tile_type,
                    quarter_rotations,
                });
                all.insert(WallType {
                    tile_type,
                    quarter_rotations,
                });

                all.insert(WallType {
                    tile_type,
                    quarter_rotations,
                });
            }
        }

        return all;
    }

    pub fn collision_shapes(&self) -> Vec<CollisionShape> {
        // Note: These aren't adjusted for rotation since our transform should do that for us.

        match self.tile_type {
            TileType::Empty => Vec::new(),
            TileType::Straight => straight_piece(),
            TileType::Corner => corner_piece(),
            TileType::Tee => tee_piece(),
        }
    }

    pub fn ports(&self, position: &Position) -> HashSet<Port> {
        ports_for_tile_type(self.tile_type)
            .iter()
            .map(|port| Port {
                port_type: port.port_type,
                position: shift_position(
                    position,
                    rotate_port_position(port.position, self.quarter_rotations),
                ),
            })
            .collect()
    }
}

fn shift_position((x, y): &Position, (dx, dy): (i16, i16)) -> Position {
    (x + dx, y + dy)
}

fn straight_piece() -> Vec<CollisionShape> {
    vec![cuboid(32.0, 16.0)]
}

fn corner_piece() -> Vec<CollisionShape> {
    vec![
        convex(vec![
            (-16.0, 16.0),
            (32.0, 16.0),
            (32.0, -16.0),
            (-16.0, -16.0),
        ]),
        convex(vec![
            (-16.0, 16.0),
            (16.0, 16.0),
            (16.0, -32.0),
            (-16.0, -32.0),
        ]),
    ]
}

fn tee_piece() -> Vec<CollisionShape> {
    vec![
        cuboid(32.0, 16.0),
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

fn ports_for_tile_type(tile_type: TileType) -> Vec<Port> {
    match tile_type {
        TileType::Empty => vec![
            Port {
                position: (0, 1),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: (1, 0),
                port_type: PortType::Empty,
            },
            Port {
                position: (0, -1),
                port_type: PortType::Empty,
            },
            Port {
                position: (-1, 0),
                port_type: PortType::Empty,
            },
        ],
        TileType::Straight => vec![
            Port {
                position: (0, 1),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: (1, 0),
                port_type: PortType::Wall,
            },
            Port {
                position: (0, -1),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: (-1, 0),
                port_type: PortType::Wall,
            },
        ],
        TileType::Corner => vec![
            Port {
                position: (0, 1),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: (1, 0),
                port_type: PortType::Wall,
            },
            Port {
                position: (0, -1),
                port_type: PortType::Wall,
            },
            Port {
                position: (-1, 0),
                port_type: PortType::EmptyRequired,
            },
        ],
        TileType::Tee => vec![
            Port {
                position: (0, 1),
                port_type: PortType::Wall,
            },
            Port {
                position: (1, 0),
                port_type: PortType::Wall,
            },
            Port {
                position: (0, -1),
                port_type: PortType::EmptyRequired,
            },
            Port {
                position: (-1, 0),
                port_type: PortType::Wall,
            },
        ],
    }
}

fn rotate_port_position((x, y): Position, rotations: QuarterRotation) -> Position {
    match rotations {
        QuarterRotation::Zero => (x, y),
        QuarterRotation::One => (-y, x),
        QuarterRotation::Two => (-x, -y),
        QuarterRotation::Three => (y, -x),
    }
}
