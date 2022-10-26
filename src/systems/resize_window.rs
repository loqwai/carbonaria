use bevy::prelude::*;

pub fn resize_window(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        window.set_resolution(512.0, 512.0);
        // window.set_maximized(true);
    }
}
