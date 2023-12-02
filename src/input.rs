use bevy::prelude::*;

use crate::camera::MainCamera;

pub struct InputPlugin;

/// A click (or touch) event.
#[derive(Event)]
pub struct ClickEvent {
    pos: Vec2,
}

#[derive(Resource, Debug)]
pub struct Mouse {
    /// Position of the mouse in world space.
    pos: Option<Vec2>,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_mouse_pos)
            .insert_resource(Mouse { pos: None })
            .add_event::<ClickEvent>();
    }
}

fn update_mouse_pos(
    mut mouse: ResMut<Mouse>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            let world_pos = ray.origin.truncate();

            // println!("MousePos is {:?}", world_pos);
            mouse.pos = Some(world_pos);
            return;
        }
    }
    mouse.pos = None;
}
