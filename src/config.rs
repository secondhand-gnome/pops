pub const GAME_TITLE: &str = "Gotta Have My Pops";
pub const RAPIER_PIXELS_PER_METER: f32 = 16.;

pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

