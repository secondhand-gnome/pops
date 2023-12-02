pub const GAME_TITLE: &str = "Gotta Have My Pops";
pub const RAPIER_PIXELS_PER_METER: f32 = 16.;

// Window resolution settings
pub const WINDOW_HEIGHT: f32 = 600.;
pub const WINDOW_WIDTH: f32 = 800.;

pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}
