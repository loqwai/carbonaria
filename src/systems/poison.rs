use bevy::prelude::*;

use crate::components::{Health, Math, Poison};
pub fn poison(mut commands: Commands, poisons: Query<(Entity, &Poison)>) {
    poisons.for_each(|(entity, poison)| {
        if poison.0 <= 0.0 {
            return;
        }

        let damage = commands.spawn(Math::add(Health(-poison.0))).id();
        commands.entity(entity).push_children(&[damage]);
    });
}

#[cfg(test)]
mod tests {
    use super::super::{powerup_defaulter, powerup_mather};
    use super::*;

    #[test]
    fn poison_subtracts_from_health() {
        let mut app = App::new();

        app.add_system(powerup_defaulter::<Health>)
            .add_system(powerup_defaulter::<Poison>)
            .add_system(powerup_mather::<Health>.after(powerup_defaulter::<Health>))
            .add_system(powerup_mather::<Poison>.after(powerup_defaulter::<Poison>))
            .add_system(poison.after(powerup_mather::<Poison>));

        let entity = app
            .world
            .spawn((Health::default(), Poison::default()))
            .with_children(|entity| {
                entity.spawn(Math::add(Health(10.0)));
                entity.spawn(Math::add(Poison(1.0)));
            })
            .id();

        app.update(); // The first update adds the the negative health
        app.update(); // The second update uses it to subtract from the health (and adds another negative health that will be used next tick)

        let remaining_health = app
            .world
            .query::<&Health>()
            .get(&app.world, entity)
            .unwrap();

        assert_eq!(&Health(9.0), remaining_health);
    }
}
