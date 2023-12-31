use bevy::prelude::*;

use crate::camera::MainCamera;

pub struct InputPlugin;

/// A click (or touch) event.
#[derive(Event)]
pub struct ClickEvent {
    pub pos: Vec2,
}

#[derive(Resource, Debug)]
pub struct Mouse {
    /// Position of the mouse in world space.
    pos: Option<Vec2>,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_mouse)
            .insert_resource(Mouse { pos: None })
            .add_event::<ClickEvent>();
    }
}

fn update_mouse(
    mut mouse: ResMut<Mouse>,
    mouse_buttons: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut ev_click: EventWriter<ClickEvent>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world(camera_transform, cursor_pos))
        .map(|ray| ray.origin.truncate())
    {
        // println!("MousePos is {:?}", world_pos);
        mouse.pos = Some(world_pos);
    } else {
        mouse.pos = None;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        match mouse.pos {
            Some(pos) => {
                debug!("Click! at {:?}", pos);
                ev_click.send(ClickEvent { pos });
            }
            None => {
                warn!("Somehow clicked with no mouse position!");
            }
        }
    }

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            debug!("Touch! at {:?}", finger.position());
            if let Some(pos) = camera
                .viewport_to_world(camera_transform, finger.position())
                .map(|ray| ray.origin.truncate())
            {
                ev_click.send(ClickEvent { pos });
            }
        }
    }
}
