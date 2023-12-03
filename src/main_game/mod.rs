mod kernel;
mod layers;
mod skillet;

use bevy::prelude::*;

use self::{kernel::KernelPlugin, skillet::SkilletPlugin};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KernelPlugin, SkilletPlugin));
    }
}
