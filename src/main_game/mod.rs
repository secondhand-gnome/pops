mod bank_account;
mod economy;
mod kernel;
mod layers;
mod skillet;
mod ui;

use bevy::prelude::*;

use self::{
    bank_account::BankAccountPlugin, kernel::KernelPlugin, skillet::SkilletPlugin, ui::UiPlugin,
};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BankAccountPlugin, KernelPlugin, SkilletPlugin, UiPlugin));
    }
}
