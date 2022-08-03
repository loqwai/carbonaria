use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};

use crate::bundles::{WallBundle, WallTexture};

pub fn spawn_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    let room = generate_room();

    for (position, tile) in room.iter() {
        let x: f32 = f32::from(position.0 .0);
        let y: f32 = f32::from(position.0 .1);

        commands.spawn_bundle(WallBundle::new(
            &asset_server,
            tile.texture,
            (x, y),
            tile.rotation,
            Vec3::new(16.0, 16.0, 0.0),
        ));
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position(pub (i16, i16));

struct Tile {
    texture: WallTexture,
    rotation: f32,
}

fn generate_room() -> HashMap<Position, Tile> {
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
