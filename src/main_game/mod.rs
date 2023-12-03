mod kernel;
mod layers;

use bevy::prelude::*;

use self::kernel::KernelPlugin;

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KernelPlugin);
    }
}
