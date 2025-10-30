use bevy::prelude::*;
use bevy::window::WindowMode;

pub fn resize_window(mut q_window: Query<&mut Window>) {
    for mut window in q_window.iter_mut() {
        window.mode = WindowMode::Windowed;
    }
}
