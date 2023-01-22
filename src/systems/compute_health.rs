use bevy::prelude::*;

use crate::components::{Health, Modifier, Modifies};
// consider genericing this to take a component type that is multipliable, etc
pub fn compute_health(
    modifiers: Query<(&Health, &Modifies), With<Modifier>>,
    mut healthies: Query<&mut Health, Without<Modifier>>,
) {
    modifiers.for_each(|(mods, modifies)| {
        let target = modifies.0;
        match healthies.get_component_mut::<Health>(target) {
            Ok(mut health) => {
                health.0 += mods.0;
            }
            Err(e) => {
                println!("Entity {:?} does not have a Health component", target);
            }
        }
    });
}

pub fn reset_computed_health_to_0(mut healthies: Query<&mut Health, Without<Modifier>>) {
    healthies.for_each_mut(|mut health| {
        health.0 = 0;
    });
}

#[test]
fn did_add_health() {
    // Setup app
    let mut app = App::new();

    // Add our two systems
    app.add_system(reset_computed_health_to_0.before(compute_health));
    app.add_system(compute_health);

    // Setup test entities
    let player = app.world.spawn(Health(555)).id();

    app.world.spawn((Health(10), Modifier, Modifies(player)));
    app.world.spawn((Health(-20), Modifier, Modifies(player)));
    // Run systems
    app.update();

    // Check resulting changes
    let hp = app.world.get::<Health>(player).unwrap();
    assert_eq!(hp.0, -10);
}

#[test]
fn modifying_health_for_entities_without_health_is_ignored() {
    // Setup app
    let mut app = App::new();

    // Add our two systems
    app.add_system(reset_computed_health_to_0.before(compute_health));
    app.add_system(compute_health);

    // Setup test entities
    let player = app.world.spawn_empty().id();

    app.world.spawn((Health(10), Modifier, Modifies(player)));
    app.world.spawn((Health(-20), Modifier, Modifies(player)));
    // Run systems
    app.update();
}
