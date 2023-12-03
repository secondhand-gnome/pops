use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{asset_loader::TextureAtlasAssets, input::ClickEvent};

use super::layers::{CollisionGroupMethods, Layer};

const KERNEL_SPRITE_SIZE_PX: Vec2 = Vec2::new(16., 16.);
const KERNEL_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

pub struct KernelPlugin;

#[derive(Event)]
struct PopEvent {
    kernel: Entity,
}

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .add_systems(Update, (click_listener, pop_kernels))
            .add_event::<PopEvent>()
            .register_type::<Kernel>()
            .register_type::<KernelState>();
    }
}

#[derive(Debug, Default, Reflect, PartialEq, Eq)]
enum KernelState {
    #[default]
    Raw,
    Popped,
}

impl KernelState {
    fn sprite_index(&self) -> usize {
        match self {
            Self::Raw => 0,
            Self::Popped => 1,
        }
    }
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
            transform: Transform::from_scale(KERNEL_SPRITE_SCALE)
                .with_translation(Vec3::Z * Layer::Kernel.z()),
            ..default()
        },
        Collider::cuboid(KERNEL_SPRITE_SIZE_PX.x / 2., KERNEL_SPRITE_SIZE_PX.y / 2.), // TODO custom collider
        RigidBody::Dynamic,
        kernel_collision_groups(),
        SolverGroups::new(kernel_group(), kernel_group()),
        Name::new("Kernel"),
    ));
}

fn click_listener(
    mut ev_click: EventReader<ClickEvent>,
    rapier_context: Res<RapierContext>,
    mut ev_pop: EventWriter<PopEvent>,
) {
    for ev in ev_click.read() {
        let filter = QueryFilter::new().groups(kernel_collision_groups());
        rapier_context.intersections_with_point(ev.pos, filter, |entity| {
            debug!("Clicked on entity {:?}", entity);
            ev_pop.send(PopEvent { kernel: entity });
            true
        });
    }
}

fn kernel_group() -> Group {
    vec![Layer::Kernel].join_groups()
}

fn kernel_collision_groups() -> CollisionGroups {
    CollisionGroups::new(kernel_group(), kernel_group())
}

fn pop_kernels(
    mut ev_pop: EventReader<PopEvent>,
    mut q_kernels: Query<(Entity, &mut Kernel, &mut TextureAtlasSprite)>,
) {
    for ev in ev_pop.read() {
        for (entity, mut kernel, mut sprite) in q_kernels.iter_mut() {
            if ev.kernel == entity && kernel.state == KernelState::Raw {
                kernel.state = KernelState::Popped;
                sprite.index = KernelState::Popped.sprite_index();
                debug!("Pop!");
            }
        }
    }
}
