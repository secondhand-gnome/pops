use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{asset_loader::TextureAtlasAssets, input::ClickEvent};

use super::layers::{CollisionGroup, CollisionGroupMethods};

const KERNEL_SPRITE_SIZE_PX: Vec2 = Vec2::new(16., 16.);
const KERNEL_SPRITE_SCALE: Vec3 = Vec3::new(2., 2., 1.);

pub struct KernelPlugin;

#[derive(Event)]
struct PopEvent(Entity);

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .add_systems(Update, (click_listener, pop_kernels))
            .add_event::<PopEvent>()
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
        // TODO make dynamic
        // RigidBody::Dynamic,
        RigidBody::Fixed,
        // CollisionGroups::new(1.into(), 1.into()),
        Name::new("Kernel"),
    ));
    commands
        .spawn(Collider::ball(0.5))
        .insert(kernel_collision_groups())
        .insert(SolverGroups::new(kernel_group(), kernel_group()));
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
            ev_pop.send(PopEvent(entity));
            true
        });
    }
}

fn kernel_group() -> Group {
    vec![CollisionGroup::Kernel].join_groups()
}

fn kernel_collision_groups() -> CollisionGroups {
    CollisionGroups::new(kernel_group(), kernel_group())
}

fn pop_kernels(
    mut commands: Commands,
    mut ev_pop: EventReader<PopEvent>,
    mut q_kernels: Query<(Entity, &mut Kernel)>,
) {
    for ev in ev_pop.read() {
        for (entity, mut kernel) in q_kernels.iter_mut() {
            // TODO
            debug!("Pop!");
        }
    }
}
