mod kernel;
mod layers;
mod skillet;
mod ui;

use bevy::prelude::*;

use self::{kernel::KernelPlugin, skillet::SkilletPlugin, ui::UiPlugin};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KernelPlugin, SkilletPlugin, UiPlugin));
    }
}
