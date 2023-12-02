use bevy::prelude::*;

use crate::asset_loader::TextureAtlasAssets;

const KERNEL_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

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

fn spawn_first_kernel(mut commands: Commands, texture_atlases: Res<TextureAtlasAssets>) {
    commands.spawn((
        Kernel { ..default() },
        SpriteSheetBundle {
            texture_atlas: texture_atlases.kernel.clone(),
            sprite: TextureAtlasSprite::new(0), // TODO indexes
            transform: Transform::from_scale(KERNEL_SPRITE_SCALE),
            ..default()
        },
        Name::new("Kernel"),
    ));
}
