use bevy::prelude::*;

use crate::config::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (spawn_camera, render_background));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn render_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(hex(BACKGROUND_COLOR)));
}
