use std::{collections::HashMap, f32::consts::PI};

use crate::bundles::WallTexture;

#[derive(PartialEq, Eq, Hash)]
pub struct Position(pub (i16, i16));

pub struct Tile {
    pub texture: WallTexture,
    pub rotation: f32,
}

pub fn generate_room() -> HashMap<Position, Tile> {
    let num_tiles: i16 = 8;
    let tile_size: usize = 64;
    let end: i16 = (num_tiles / 2) * i16::try_from(tile_size).unwrap();
    let begin: i16 = end * -1;

    let mut map: HashMap<Position, Tile> = HashMap::new();

    for i in (begin..end).step_by(tile_size) {
        let x = i;
        let y = i;

        match x {
            // corner sections
            x if x == begin => {
                map.insert(
                    Position((begin, begin)),
                    Tile {
                        texture: WallTexture::Corner,
                        rotation: PI / -2.0,
                    },
                );

                map.insert(
                    Position((begin, end)),
                    Tile {
                        texture: WallTexture::Corner,
                        rotation: PI,
                    },
                );
                map.insert(
                    Position((end, end)),
                    Tile {
                        texture: WallTexture::Corner,
                        rotation: PI / 2.0,
                    },
                );

                map.insert(
                    Position((end, begin)),
                    Tile {
                        texture: WallTexture::Corner,
                        rotation: 0.0,
                    },
                );
            }

            // normal straight wall section
            _ => {
                map.insert(
                    Position((x, begin)),
                    Tile {
                        texture: WallTexture::Straight,
                        rotation: 0.0,
                    },
                );

                map.insert(
                    Position((x, end)),
                    Tile {
                        texture: WallTexture::Straight,
                        rotation: 0.0,
                    },
                );

                map.insert(
                    Position((begin, y)),
                    Tile {
                        texture: WallTexture::Straight,
                        rotation: PI / 2.0,
                    },
                );

                map.insert(
                    Position((end, y)),
                    Tile {
                        texture: WallTexture::Straight,
                        rotation: PI / 2.0,
                    },
                );
            }
        }
    }

    map
}
