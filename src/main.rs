mod asset_loader;
mod camera;
mod config;
mod input;
mod main_game;

use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::MainCamera;

use crate::config::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(get_window_plugin())
            .set(ImagePlugin::default_nearest()),
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(RAPIER_PIXELS_PER_METER),
        AudioPlugin,
        asset_loader::AssetLoaderPlugin,
        camera::CameraPlugin,
        main_game::MainGamePlugin,
        // TODO add ui
    ));

    if config::is_debug() {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default(),
        ));
    }

    app.run();
}

fn get_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            title: GAME_TITLE.into(),
            ..default()
        }),
        ..default()
    }
}
