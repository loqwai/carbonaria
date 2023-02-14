use bevy::prelude::*;

use crate::components::{Health, Math, Poison, TimeToLive};
pub fn poison(mut commands: Commands, q_poison: Query<(Entity, &Poison)>) {
    q_poison.for_each(|(entity, poison)| {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(Math::add(Health(-1))).with_children(|p2| {
                p2.spawn(Math::add(TimeToLive(10)));
            });
        });
    });
}

#[test]
fn did_poison_work() {
    use super::{powerup_defaulter, attach_time_to_live, powerup_mather};
    let mut app = App::new();
    let powerup_set = SystemSet::new().label("powerup")
        .with_system(powerup_defaulter::<Health>)
        .with_system(powerup_defaulter::<TimeToLive>)
        .with_system(powerup_mather::<Health>.after(powerup_defaulter::<Health>))
        .with_system(powerup_mather::<TimeToLive>.after(powerup_defaulter::<TimeToLive>));


    app.add_system_set(powerup_set);

    let game_loop_set = SystemSet::new()
        .with_system(attach_time_to_live)
        .with_system(poison);

    app.add_system_set(game_loop_set.after("powerup"));
    
    let poisoned = app.world.spawn_empty().insert(Poison(1));
    app.update();

    let (entity, health_math) = app
        .world
        .query::<(Entity, &Math<Health>)>()
        .iter(&app.world)
        .next()
        .unwrap();
    assert_eq!(health_math.add, Some(Health(-1)));

    let (parent,_) = app
        .world
        .query::<(&Parent, &Math<TimeToLive>)>()
        .iter(&app.world)
        .next()
        .unwrap();
    assert_eq!(parent.get(), entity);

    app.update();


}
