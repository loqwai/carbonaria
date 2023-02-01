use bevy::prelude::*;
use crate::components::Health;
pub fn health_powerup_add_health(
  mut commands: Commands,
  powerups: Query<(Entity, &Parent, &Health)>, mut parent_healths: Query<&mut Health, Without<Parent>>) {

  powerups.for_each(|(entity, parent, powerup_health)| {
      match parent_healths.get_mut(parent.get()) {
          Ok(mut parent_health) => {
            parent_health.0 += powerup_health.0;            
            commands.entity(entity).despawn_recursive();

          },
          Err(_) => panic!("Somehow we found a powerup that belongs to a parent that doesn't exist"),
      }
  });
}
