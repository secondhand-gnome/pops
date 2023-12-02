use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{asset_loader::TextureAtlasAssets, input::ClickEvent};

const KERNEL_SPRITE_SIZE_PX: Vec2 = Vec2::new(16., 16.);
const KERNEL_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

pub struct KernelPlugin;

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .add_systems(Update, click_listener)
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
        Collider::cuboid(KERNEL_SPRITE_SIZE_PX.x / 2., KERNEL_SPRITE_SIZE_PX.y / 2.), // TODO custom collider
        RigidBody::Dynamic,
        Name::new("Kernel"),
    ));
}

fn click_listener(ev_click: EventReader<ClickEvent>) {
    // TODO respond to clicks
}
