use bevy::prelude::*;

pub struct AutoKettlePlugin;

#[derive(Event)]
pub struct AutoKettlePurchaseEvent;

impl Plugin for AutoKettlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buy_auto_kettle)
            .add_event::<AutoKettlePurchaseEvent>();
    }
}

fn buy_auto_kettle(mut ev_buy_auto_kettle: EventReader<AutoKettlePurchaseEvent>) {
    for _ in ev_buy_auto_kettle.read() {
        //
        println!("TODO create a kettle");
    }
}
