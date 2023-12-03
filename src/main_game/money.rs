// TODO rename to bank_account
use bevy::prelude::*;
use bigdecimal::{
    num_bigint::{BigInt, ToBigInt},
    BigDecimal, FromPrimitive, ToPrimitive,
};
use num_format::{Locale, ToFormattedString};
use std::{fmt, ops::Mul};

use super::kernel::KernelPurchaseEvent;

const INITIAL_ACCOUNT_BALANCE: f32 = 1.00;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BankAccount::default())
            .add_systems(Startup, initial_bank_credit)
            .add_systems(Update, kernel_purchase_listener);
    }
}

#[derive(Resource, Default)]
pub struct BankAccount {
    balance: BigDecimal,
}

impl BankAccount {
    pub fn credit(&mut self, amount: f32) {
        self.balance += BigDecimal::from_f32(amount).unwrap().round(2);
    }

    pub fn debit(&mut self, amount: f32) {
        self.balance -= BigDecimal::from_f32(amount).unwrap().round(2);
    }

    pub fn has_at_least(&self, amount: f32) -> bool {
        self.balance >= BigDecimal::from_f32(amount).unwrap()
    }

    /// Return the dollars part of the balance.
    fn whole_dollars(&self) -> BigInt {
        self.balance.to_bigint().unwrap()
    }

    /// Return the cents part of the balance.
    fn cents(&self) -> u8 {
        (self.balance.clone() % BigDecimal::from(1))
            .mul(BigDecimal::from(100))
            .to_u8()
            .unwrap()
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "${}.{:0>2}",
            self.whole_dollars().to_formatted_string(&Locale::en),
            self.cents()
        )
    }
}

fn initial_bank_credit(mut bank_account: ResMut<BankAccount>) {
    bank_account.credit(INITIAL_ACCOUNT_BALANCE);
}

fn kernel_purchase_listener(
    mut ev_buy_kernel: EventReader<KernelPurchaseEvent>,
    mut bank_account: ResMut<BankAccount>,
) {
    for ev in ev_buy_kernel.read() {
        // TODO derive kernel price from an Economy class
        let kernel_price = 0.01;
        bank_account.debit(ev.quantity as f32 * kernel_price);
    }
}
