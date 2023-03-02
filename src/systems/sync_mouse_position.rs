use bevy::{prelude::*, render::camera::RenderTarget, window::Windows};

use crate::components::MousePos;

pub fn sync_mouse_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_mouse: Query<&mut Transform, With<MousePos>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        let Some(ray) = camera.viewport_to_world(camera_transform, screen_pos) else { return; };
        let Some(point) = get_point_on_plane(ray, Vec3::ZERO, Vec3::Z) else { return; };

        q_mouse.for_each_mut(|mut mouse| {
            mouse.translation = point;
        });
    }
}

fn intersect_plane(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<f32> {
    let denominator = plane_normal.dot(ray.direction);
    if denominator.abs() > f32::EPSILON {
        let distance = (plane_origin - ray.origin).dot(plane_normal) / denominator;
        if distance >= f32::EPSILON {
            return Some(distance);
        }
    }
    None
}

fn get_point_on_plane(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let distance = intersect_plane(ray, plane_origin, plane_normal)?;
    Some(ray.origin + ray.direction * distance)
}
