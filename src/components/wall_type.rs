use std::collections::HashSet;

use bevy::prelude::{Component, Vec3};
use heron::CollisionShape;

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WallType {
    Empty,
    Vertical,
    Horizontal,
    TopLeftCorner,
    TopRightCorner,
    BottomRightCorner,
    BottomLeftCorner,
}

type Position = (i16, i16);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Port {
    pub position: Position,
    pub port_type: PortType,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PortType {
    Empty,
    EmptyRequired,
    Wall,
}

impl WallType {
    /// all returns a hashset of all of the possible walltypes
    pub fn all() -> HashSet<WallType> {
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

    pub fn collision_shapes(&self) -> Vec<CollisionShape> {
        // Note: These aren't adjusted for rotation since our transform should do that for us.

        match self {
            WallType::Empty => Vec::new(),
            WallType::Vertical => straight_piece(),
            WallType::Horizontal => straight_piece(),
            WallType::TopLeftCorner => corner_piece(),
            WallType::TopRightCorner => corner_piece(),
            WallType::BottomRightCorner => corner_piece(),
            WallType::BottomLeftCorner => corner_piece(),
        }
    }
    pub fn ports(&self, position: &Position) -> HashSet<Port> {
        match self {
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
