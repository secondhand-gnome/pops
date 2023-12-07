use bevy::prelude::*;

pub const ENABLE_CHEATS: bool = false;
pub const ENABLE_AUTO_KETTLE: bool = false;

pub const BACKGROUND_COLOR: &str = "#3c5e8b";
pub const GAME_TITLE: &str = "Gotta Have My Pops";
pub const RAPIER_PIXELS_PER_METER: f32 = 2000.;

// Window resolution settings
pub const WINDOW_HEIGHT: f32 = 600.;
pub const WINDOW_WIDTH: f32 = 800.;

pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

pub fn hex(hex: &str) -> Color {
    Color::hex(hex).expect("color")
}
