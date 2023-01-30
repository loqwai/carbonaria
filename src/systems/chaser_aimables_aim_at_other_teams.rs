use bevy::prelude::*;
use crate::{events::RotateEvent, util::look_at_target, components::{ Chases, Aimable, Team}};

pub fn chaser_aimables_aim_at_other_teams(
    chasers: Query<(&Children, &Team), With<Chases>>,
    aimables: Query<(Entity, &GlobalTransform), With<Aimable>>,
    targets: Query<(&Team, &Transform)>,
    mut rotate_events: EventWriter<RotateEvent>,
) {
    chasers.for_each(|(chaser_children, chaser_team)| {
        if let Some((_, target_transform)) = targets.iter().find(|(team, _)| team != &chaser_team) {
          chaser_children
              .iter()
              .filter_map(|&child| aimables.get(child).ok())
              .for_each(|(aimable, aimable_transform)| {
                  let (rotation, _) =
                      look_at_target(aimable_transform.translation(), target_transform.translation);

                  rotate_events.send(RotateEvent {
                      who: aimable,
                      rotation,
                  })
              })
        }
    })
}
