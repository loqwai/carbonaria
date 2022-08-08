use bevy::prelude::*;
use heron::CollisionShape;

use crate::{
    bundles::WallBundle,
    components::{Room, Tile, WallType},
    resources::Config,
};

pub fn spawn_room(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    let room = Room::new(config.dimensions);

    commands.spawn().insert(room.clone());

    for (position, tile) in room.iter() {
        match tile {
            Tile::Options(_) => (),
            Tile::WallType(wall_type) => {
                let wall = commands
                    .spawn_bundle(WallBundle::new(&asset_server, &wall_type, *position))
                    .id();

                for shape in collision_shapes_for_wall_type(&wall_type) {
                    let child = commands.spawn().insert(shape).id();
                    commands.entity(wall).push_children(&[child]);
                }
            }
        }
    }
}

fn collision_shapes_for_wall_type(wall_type: &WallType) -> Vec<CollisionShape> {
    // Note: These aren't adjusted for rotation since our transform should do that for us.

    match wall_type {
        WallType::Empty => Vec::new(),
        WallType::Vertical => straight_piece(),
        WallType::Horizontal => straight_piece(),
        WallType::TopLeftCorner => corner_piece(),
        WallType::TopRightCorner => corner_piece(),
        WallType::BottomRightCorner => corner_piece(),
        WallType::BottomLeftCorner => corner_piece(),
    }
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
// fn collision_shape_for_wall_type(wall_type: &WallType) -> CollisionShape {
//     //     CollisionShape::ConvexHull { points: (), border_radius: () }

//     //   CollisionShape::Cuboid {
//     //                 half_extends,
//     //                 border_radius: None,
//     //             }
//     // ColliderBuilder::compound(Vec::new()).build()

//     // ColliderBuilder::cuboid(16.0, 16.0).build()
//     CollisionShape::Cuboid {
//         half_extends: Vec3::new(16.0, 16.0, 0.0),
//         border_radius: None,
//     }
// }
