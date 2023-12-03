use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

use crate::asset_loader::TextureAssets;

pub struct SkilletPlugin;

// const SKILLET_SPRITE_SIZE_PX: Vec2 = Vec2::new(256., 64.);
const SKILLET_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

impl Plugin for SkilletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_skillet);
    }
}

fn spawn_skillet(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: texture_assets.skillet.clone(),
            transform: Transform::from_scale(SKILLET_SPRITE_SCALE)
                .with_translation(Vec3::NEG_Y * SKILLET_SPRITE_SCALE * 16.),
            ..default()
        },
        // TODO collider
        skillet_collider(),
    ));
}

fn skillet_collider() -> Collider {
    Collider::polyline(
        vec![
            Vec2::new(-100., 128.),
            Vec2::new(-100., -4.),
            Vec2::new(100., -4.),
            Vec2::new(100., 128.),
        ],
        None,
    )
}
