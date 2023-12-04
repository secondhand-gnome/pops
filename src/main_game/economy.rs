use bevy::prelude::*;

pub struct EconomyPlugin;

#[derive(Resource)]
pub struct PriceChecker;

impl PriceChecker {
    pub fn raw_kernels(&self, quantity: u64) -> f32 {
        // TODO incorporate discounts based on state
        let q = quantity as f32;
        if q >= 10. {
            q * 0.008
        } else {
            q * 0.01
        }
    }
}

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PriceChecker);
    }
}
