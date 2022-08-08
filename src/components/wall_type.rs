use bevy::prelude::Component;

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
