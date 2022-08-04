use std::collections::HashMap;

use crate::bundles::WallType;

type Position = (i16, i16);

pub fn generate_room() -> HashMap<Position, WallType> {
    let num_tiles: i16 = 8;
    let end: i16 = num_tiles / 2;
    let begin: i16 = end * -1;

    let mut map: HashMap<Position, WallType> = HashMap::new();

    for i in begin..end {
        let x = i;
        let y = i;

        match x {
            // corner sections
            x if x == begin => {
                map.insert((begin, end), WallType::TopLeftCorner);
                map.insert((end, end), WallType::TopRightCorner);
                map.insert((end, begin), WallType::BottomRightCorner);
                map.insert((begin, begin), WallType::BottomLeftCorner);
            }

            // normal straight wall section
            _ => {
                map.insert((x, begin), WallType::Vertical);
                map.insert((x, end), WallType::Vertical);
                map.insert((begin, y), WallType::Horizontal);
                map.insert((end, y), WallType::Horizontal);
            }
        }
    }

    map
}
