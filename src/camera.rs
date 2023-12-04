use bevy::prelude::*;

use crate::config::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct ZoomOutEvent;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (spawn_camera, render_background))
            .add_systems(Update, zoom_out_listener)
            .add_event::<ZoomOutEvent>();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn render_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(hex(BACKGROUND_COLOR)));
}

fn zoom_out_listener(
    mut ev_zoom_out: EventReader<ZoomOutEvent>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    for _ in ev_zoom_out.read() {
        let mut camera_transform = q_camera.single_mut();
        camera_transform.scale *= 2.;
    }
}
