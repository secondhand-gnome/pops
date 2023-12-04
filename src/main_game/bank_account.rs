use bevy::prelude::*;
use bigdecimal::{
    num_bigint::{BigInt, ToBigInt},
    BigDecimal, FromPrimitive, ToPrimitive,
};
use num_format::{Locale, ToFormattedString};
use std::{fmt, ops::Mul};

const INITIAL_ACCOUNT_BALANCE: f32 = 1.00;

pub struct BankAccountPlugin;

impl Plugin for BankAccountPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BankAccount::default())
            .add_systems(Startup, initial_bank_credit);
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
            .abs()
            .to_u8()
            .unwrap()
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let neg_sign = if self.has_at_least(0.) { "" } else { "-" };
        let dollars = self.whole_dollars().to_formatted_string(&Locale::en);
        write!(f, "{neg_sign}${dollars}.{:0>2}", self.cents())
    }
}

fn initial_bank_credit(mut bank_account: ResMut<BankAccount>) {
    bank_account.credit(INITIAL_ACCOUNT_BALANCE);
}
