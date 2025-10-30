use bevy::prelude::*;

use crate::components::Team;

pub fn team_powerup_assigns_team(powerups: Query<(&Parent, &Team)>, mut parent_teams: Query<&mut Team, Without<Parent>>) {
    for (parent, powerup_team) in powerups.iter() {
        match parent_teams.get_mut(parent.get()) {
            Ok(mut parent_team) => parent_team.0 = powerup_team.0,
            Err(_) => panic!("Somehow we found a powerup that belongs to a parent that doesn't exist"),
        }
    }
}
