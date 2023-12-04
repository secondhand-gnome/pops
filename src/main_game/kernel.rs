use std::{f32::consts::PI, ops::Range};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bigdecimal::num_bigint::BigInt;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::fmt;

use crate::{asset_loader::TextureAtlasAssets, input::ClickEvent};

use super::{
    bank_account::BankAccount,
    economy::PriceChecker,
    layers::{CollisionGroupMethods, Layer},
};

pub const POSSIBLE_KERNEL_BUY_QUANTITIES: [u64; 3] = [1, 10, 100];
pub const POSSIBLE_SELL_QUANTITIES: [u64; 5] = [100, 500, 1000, 10000, 100000];

const KERNEL_SPRITE_SIZE_PX: Vec2 = Vec2::new(16., 16.);
const KERNEL_SPRITE_SCALE_RAW: Vec3 = Vec3::new(1., 1., 1.);
const KERNEL_SPRITE_SCALE_POPPED: Vec3 = Vec3::new(2., 2., 1.);

// See article "Physical properties of popcorn kernels"
// https://www.sciencedirect.com/science/article/abs/pii/S0260877404006016

const KERNEL_SPAWN_LOCATION_X_RANGE: Range<f32> = (-80.)..(80.);

// A kernel weighs 0.15 grams
const KERNEL_MASS: f32 = 0.15;

// A kernel is 6mm in diameter

pub struct KernelPlugin;

#[derive(Event)]
pub struct KernelPurchaseEvent {
    pub quantity: u64,
}

#[derive(Event)]
pub struct KernelSpawnEvent {
    pub quantity: u64,
}

#[derive(Event)]
pub struct PopcornSellEvent {
    pub quantity: u64,
}

#[derive(Event)]
struct PopEvent {
    kernel: Entity,
}

#[derive(Resource, Default)]
pub struct PopCounter {
    count: BigInt,
}

#[derive(Component)]
struct Popcorn;

impl PopCounter {
    fn count_pop(&mut self) {
        self.count += BigInt::from(1);
    }
}

#[derive(Resource, Default)]
/// Holds the current amount of popped kernels that have not yet been sold.
pub struct PopcornCounter {
    quantity: i64,
}

impl PopcornCounter {
    pub fn quantity(&self) -> i64 {
        self.quantity
    }
}

impl fmt::Display for PopCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl Plugin for KernelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_kernel)
            .add_systems(
                Update,
                (
                    click_listener,
                    pop_kernels,
                    spawn_kernels,
                    kernel_purchase_listener,
                    sell_popcorn,
                ),
            )
            .add_event::<KernelPurchaseEvent>()
            .add_event::<KernelSpawnEvent>()
            .add_event::<PopEvent>()
            .add_event::<PopcornSellEvent>()
            .insert_resource(PopCounter::default())
            .insert_resource(PopcornCounter::default())
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

fn spawn_first_kernel(mut ev_spawn_kernel: EventWriter<KernelSpawnEvent>) {
    ev_spawn_kernel.send(KernelSpawnEvent { quantity: 1 });
}

fn spawn_kernels(
    mut commands: Commands,
    mut ev_spawn_kernel: EventReader<KernelSpawnEvent>,
    texture_atlases: Res<TextureAtlasAssets>,
) {
    for ev in ev_spawn_kernel.read() {
        for _ in 0..ev.quantity {
            let mut rng = rand::thread_rng();
            let translation = Vec3 {
                x: rng.gen_range(KERNEL_SPAWN_LOCATION_X_RANGE),
                y: 0.,
                z: Layer::RawKernel.z(),
            };
            commands.spawn((
                Kernel { ..default() },
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.kernel.clone(),
                    sprite: TextureAtlasSprite::new(0), // TODO indexes
                    transform: Transform::from_scale(KERNEL_SPRITE_SCALE_RAW)
                        .with_translation(translation),
                    ..default()
                },
                Collider::cuboid(KERNEL_SPRITE_SIZE_PX.x / 2., KERNEL_SPRITE_SIZE_PX.y / 2.), // TODO custom collider
                ColliderMassProperties::Mass(KERNEL_MASS),
                RigidBody::Dynamic,
                CollisionGroups::new(vec![Layer::RawKernel].group(), vec![Layer::Skillet].group()),
                Name::new("Kernel"),
            ));
        }
    }
}

fn click_listener(
    mut ev_click: EventReader<ClickEvent>,
    rapier_context: Res<RapierContext>,
    mut ev_pop: EventWriter<PopEvent>,
) {
    for ev in ev_click.read() {
        let filter = QueryFilter::new().groups(CollisionGroups::new(
            Group::ALL,
            vec![Layer::RawKernel].group(),
        ));
        rapier_context.intersections_with_point(ev.pos, filter, |entity| {
            debug!("Clicked on entity {:?}", entity);
            ev_pop.send(PopEvent { kernel: entity });
            true
        });
    }
}

fn pop_kernels(
    mut commands: Commands,
    mut ev_pop: EventReader<PopEvent>,
    mut pop_counter: ResMut<PopCounter>,
    mut popcorn_counter: ResMut<PopcornCounter>,
    mut q_kernels: Query<(
        Entity,
        &mut Kernel,
        &mut TextureAtlasSprite,
        &mut CollisionGroups,
        &mut Transform,
    )>,
) {
    for ev in ev_pop.read() {
        for (entity, mut kernel, mut sprite, mut collision_groups, mut transform) in
            q_kernels.iter_mut()
        {
            if ev.kernel == entity && kernel.state == KernelState::Raw {
                pop_counter.count_pop();
                popcorn_counter.quantity += 1;

                // Change the kernel's state to Popped
                kernel.state = KernelState::Popped;

                // Change the kernel's sprite
                sprite.index = KernelState::Popped.sprite_index();

                // Change the kernel's collision groups
                // It's a popped kernel
                collision_groups.memberships = vec![Layer::PoppedKernel].group();
                // that collides with other popped kernels.
                collision_groups.filters |= vec![Layer::PoppedKernel].group();

                // Apply an impulse to the kernel
                commands
                    .entity(entity)
                    .insert((kernel_pop_impulse(), Popcorn));

                // Change the scale
                transform.scale = KERNEL_SPRITE_SCALE_POPPED;

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

fn kernel_purchase_listener(
    mut ev_buy_kernel: EventReader<KernelPurchaseEvent>,
    price_checker: Res<PriceChecker>,
    mut bank_account: ResMut<BankAccount>,
) {
    for ev in ev_buy_kernel.read() {
        bank_account.debit(price_checker.raw_kernels(ev.quantity));
    }
}

fn sell_popcorn(
    mut commands: Commands,
    mut ev_sell_popcorn: EventReader<PopcornSellEvent>,
    mut popcorn_counter: ResMut<PopcornCounter>,
    mut bank_account: ResMut<BankAccount>,
    price_checker: Res<PriceChecker>,
    q_popcorn: Query<Entity, With<Popcorn>>,
) {
    for ev in ev_sell_popcorn.read() {
        // Subtract from the popcorn stockpile
        popcorn_counter.quantity -= ev.quantity as i64;

        // Credit the bank account
        bank_account.credit(price_checker.popcorn(ev.quantity));

        // Despawn popcorn entities
        for (index, entity) in q_popcorn.iter().enumerate() {
            if (index as u64) < ev.quantity {
                commands.entity(entity).despawn();
            }
        }
    }
}
