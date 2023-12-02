mod asset_loader;
mod common;
mod config;
mod input;
mod main_game;

use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use common::MainCamera;

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
    ))
    .add_systems(Startup, (spawn_camera, render_background));

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

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn render_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(hex(BACKGROUND_COLOR)));
}
