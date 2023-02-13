use bevy::prelude::*;
use crate::components::MousePos;
pub fn spawn_crosshairs(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("crosshairs.png"),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    }).insert(MousePos);
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
}
