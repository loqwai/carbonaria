use std::collections::HashMap;

use crate::bundles::WallType;

#[derive(PartialEq, Eq, Hash)]
pub struct Position(pub (i16, i16));

pub fn generate_room() -> HashMap<Position, WallType> {
    let num_tiles: i16 = 8;
    let tile_size: usize = 64;
    let end: i16 = (num_tiles / 2) * i16::try_from(tile_size).unwrap();
    let begin: i16 = end * -1;

    let mut map: HashMap<Position, WallType> = HashMap::new();

    for i in (begin..end).step_by(tile_size) {
        let x = i;
        let y = i;

        match x {
            // corner sections
            x if x == begin => {
                map.insert(Position((begin, end)), WallType::TopLeftCorner);
                map.insert(Position((end, end)), WallType::TopRightCorner);
                map.insert(Position((end, begin)), WallType::BottomRightCorner);
                map.insert(Position((begin, begin)), WallType::BottomLeftCorner);
            }

            // normal straight wall section
            _ => {
                map.insert(Position((x, begin)), WallType::Vertical);
                map.insert(Position((x, end)), WallType::Vertical);
                map.insert(Position((begin, y)), WallType::Horizontal);
                map.insert(Position((end, y)), WallType::Horizontal);
            }
        }
    }

    map
}
