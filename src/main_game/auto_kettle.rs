use bevy::prelude::*;

use crate::asset_loader::TextureAssets;

use super::{bank_account::BankAccount, economy::PriceChecker, layers::Layer};

pub struct AutoKettlePlugin;

const SPRITE_SCALE: Vec3 = Vec3::new(8., 8., 1.);

#[derive(Event)]
pub struct AutoKettlePurchaseEvent;

impl Plugin for AutoKettlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buy_auto_kettle)
            .add_event::<AutoKettlePurchaseEvent>();
    }
}

// TODO - create collider, offset, zoom out camera, and start popping kernels in the background
fn buy_auto_kettle(
    mut commands: Commands,
    mut ev_buy_auto_kettle: EventReader<AutoKettlePurchaseEvent>,
    mut bank_account: ResMut<BankAccount>,
    price_checker: Res<PriceChecker>,
    texture_assets: Res<TextureAssets>,
) {
    for _ in ev_buy_auto_kettle.read() {
        commands.spawn(
            (SpriteBundle {
                texture: texture_assets.auto_kettle.clone(),
                transform: Transform::from_translation(Vec3::Z * Layer::Skillet.z())
                    .with_scale(SPRITE_SCALE),
                ..default()
            }),
        );

        bank_account.debit(price_checker.auto_kettle());
    }
}
