use bevy::{prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::components::*;
pub fn add_inspector(app: &mut App) {
    app.
    add_plugin(WorldInspectorPlugin::new())
    .register_type::<Points>()
    .register_type::<Health>()
    .register_type::<Speed>()
    .register_type::<Chest>()
    ;
}
