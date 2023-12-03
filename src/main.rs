mod asset_loader;
mod camera;
mod config;
mod input;
mod main_game;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

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
        input::InputPlugin,
        main_game::MainGamePlugin,
        // TODO add ui
    ));

    // Fix itch.io 403 error - see https://github.com/bevyengine/bevy/pull/10623
    app.insert_resource(AssetMetaCheck::Never);

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
