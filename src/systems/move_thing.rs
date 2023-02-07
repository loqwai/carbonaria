use bevy::prelude::*;

use crate::{components::Speed, events::MoveEvent};

pub fn move_thing(
    mut moveable: Query<(&mut Transform, &Speed)>,
    // separate children so we can still move things without children
    moveable_children: Query<&Children, (With<Transform>, With<Speed>)>,

    speed_query: Query<&Speed>,
    mut move_events: EventReader<MoveEvent>,
) {
    move_events.iter().for_each(|event| {
        if event.direction == Vec3::default() {
            return; // We can't call normalize on a 0,0,0 vector
        }

        match moveable.get_mut(event.who) {
            Err(_) => return,
            Ok((mut transform, own_speed)) => {
                let entity_speed: f32 = own_speed.0;
                let speed_multiplier =
                    get_speed_multiplier(&moveable_children, &speed_query, event.who)
                        .unwrap_or(1.0);

                transform.translation +=
                    event.direction.normalize() * entity_speed * speed_multiplier;
            }
        };
    });
}

fn get_speed_multiplier(
    moveable_children: &Query<&Children, (With<Transform>, With<Speed>)>,
    speed_query: &Query<&Speed>,
    parent: Entity,
) -> Option<f32> {
    let children = moveable_children.get(parent).ok()?;

    if children.iter().count() == 0 {
        return None;
    }

    Some(
        children
            .iter()
            .filter_map(|&child| speed_query.get(child).ok())
            .map(|speed| speed.0)
            .product(),
    )
}
