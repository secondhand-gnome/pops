use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;
use bevy_rapier2d::{parry::math::AngularInertia, prelude::*};
use rand::Rng;

use crate::{asset_loader::TextureAtlasAssets, input::ClickEvent};

use super::layers::{CollisionGroupMethods, Layer};

const KERNEL_SPRITE_SIZE_PX: Vec2 = Vec2::new(16., 16.);
const KERNEL_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

// See article "Physical properties of popcorn kernels"
// https://www.sciencedirect.com/science/article/abs/pii/S0260877404006016

const KERNEL_SPAWN_LOCATION_X_RANGE: Range<f32> = (-80.)..(80.);

// A kernel weighs 0.15 grams
const KERNEL_MASS: f32 = 0.15;

// A kernel is 6mm in diameter

pub struct KernelPlugin;

#[derive(Event)]
pub struct SpawnKernelEvent;

#[derive(Event)]
struct PopEvent {
    kernel: Entity,
}

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .add_systems(Update, (click_listener, pop_kernels, spawn_kernels))
            .add_event::<PopEvent>()
            .add_event::<SpawnKernelEvent>()
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

fn spawn_first_kernel(mut ev_spawn_kernel: EventWriter<SpawnKernelEvent>) {
    ev_spawn_kernel.send(SpawnKernelEvent);
}

fn spawn_kernels(
    mut commands: Commands,
    mut ev_spawn_kernel: EventReader<SpawnKernelEvent>,
    texture_atlases: Res<TextureAtlasAssets>,
) {
    for _ in ev_spawn_kernel.read() {
        let mut rng = rand::thread_rng();
        let translation = Vec3 {
            x: rng.gen_range(KERNEL_SPAWN_LOCATION_X_RANGE),
            y: 0.,
            z: Layer::Kernel.z(),
        };
        commands.spawn((
            Kernel { ..default() },
            SpriteSheetBundle {
                texture_atlas: texture_atlases.kernel.clone(),
                sprite: TextureAtlasSprite::new(0), // TODO indexes
                transform: Transform::from_scale(KERNEL_SPRITE_SCALE).with_translation(translation),
                ..default()
            },
            Collider::cuboid(KERNEL_SPRITE_SIZE_PX.x / 2., KERNEL_SPRITE_SIZE_PX.y / 2.), // TODO custom collider
            ColliderMassProperties::Mass(KERNEL_MASS),
            RigidBody::Dynamic,
            kernel_collision_groups(),
            SolverGroups::new(kernel_group(), kernel_group()),
            Name::new("Kernel"),
        ));
    }
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
    mut commands: Commands,
    mut ev_pop: EventReader<PopEvent>,
    mut q_kernels: Query<(Entity, &mut Kernel, &mut TextureAtlasSprite)>,
) {
    for ev in ev_pop.read() {
        for (entity, mut kernel, mut sprite) in q_kernels.iter_mut() {
            if ev.kernel == entity && kernel.state == KernelState::Raw {
                // Change the kernel's state to Popped
                kernel.state = KernelState::Popped;

                // Change the kernel's sprite
                sprite.index = KernelState::Popped.sprite_index();

                // Apply an impulse to the kernel
                commands.entity(entity).insert(kernel_pop_impulse());

                debug!("Pop!");
            }
        }
    }
}

fn kernel_pop_impulse() -> ExternalImpulse {
    const MAGNITUDE: f32 = 30.;
    const DIRECTION_RANGE: Range<f32> = (PI / 6.)..(PI * 5. / 6.);

    let mut rng = rand::thread_rng();

    ExternalImpulse {
        impulse: MAGNITUDE * Vec2::from_angle(rng.gen_range(DIRECTION_RANGE)),
        torque_impulse: 0., // TODO add torque based on angle
    }
}
