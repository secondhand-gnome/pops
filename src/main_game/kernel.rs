use bevy::prelude::*;

use crate::asset_loader::TextureAssets;

pub struct KernelPlugin;

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .register_type::<Kernel>()
            .register_type::<KernelState>();
    }
}

#[derive(Debug, Default, Reflect)]
enum KernelState {
    #[default]
    Raw,
    Popped,
}

#[derive(Component, Debug, Default, Reflect)]
struct Kernel {
    state: KernelState,
}

fn spawn_first_kernel(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Kernel { ..default() },
        SpriteBundle {
            texture: texture_assets.raw_kernel.clone(),
            ..default()
        },
        Name::new("Kernel"),
    ));
}
