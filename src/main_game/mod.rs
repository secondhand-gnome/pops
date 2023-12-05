mod auto_kettle;
mod bank_account;
mod economy;
mod kernel;
mod layers;
mod skillet;
mod ui;

use bevy::prelude::*;

use self::{
    auto_kettle::AutoKettlePlugin, bank_account::BankAccountPlugin, economy::EconomyPlugin,
    kernel::KernelPlugin, skillet::SkilletPlugin, ui::UiPlugin,
};

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AutoKettlePlugin,
            BankAccountPlugin,
            EconomyPlugin,
            KernelPlugin,
            SkilletPlugin,
            UiPlugin,
        ));
    }
}
