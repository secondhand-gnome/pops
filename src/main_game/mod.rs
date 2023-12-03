mod kernel;
mod layers;
mod money;
mod skillet;
mod ui;

use bevy::prelude::*;

use self::{kernel::KernelPlugin, money::MoneyPlugin, skillet::SkilletPlugin, ui::UiPlugin};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KernelPlugin, MoneyPlugin, SkilletPlugin, UiPlugin));
    }
}
