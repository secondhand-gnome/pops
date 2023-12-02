mod asset_loader;
mod config;
mod main_game;

use bevy::prelude::*;
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
            title: GAME_TITLE.into(),
            ..default()
        }),
        ..default()
    }
}
