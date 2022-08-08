use std::collections::{hash_map::Iter, HashMap, HashSet};

use bevy::prelude::*;

use super::WallType;

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
}

pub type Position = (i16, i16);
