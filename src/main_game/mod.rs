mod kernel;

use bevy::prelude::*;

use crate::config::hex;

use self::kernel::KernelPlugin;

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KernelPlugin)
            .add_systems(Startup, (spawn_camera, render_background));
    }
}

fn render_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(hex("#090a14")));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
