use bevy::prelude::*;
use crate::components::{BeeSting, Instigator,Position};

pub fn bee_sting(
    mut commands: Commands,
    bee_stings: Query<(&BeeSting, &Instigator)>,
    parents: Query<&Children, Or<(&Parent, &Position)>>,
    instigator_positions: Query<&Position, Option<Parent>>,
) {
  // find position of instigator by recursively looking up the instigator's parent until one has a position
}
