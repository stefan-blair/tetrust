pub mod tetriminos;

pub mod settings {
    pub const QUEUE_LENGTH: usize = 3;
}

pub mod dimensions {
    pub const CELL_WIDTH: usize = 10;
    pub const CELL_HEIGHT: usize = 20;
}

pub mod gravity {
    pub const GRAVITY_RATES_IN_SECONDS: &[f32] = &[
        1.00000 * 60.0,
        0.79300 * 60.0,
        0.61780 * 60.0,
        0.47273 * 60.0,
        0.35520 * 60.0,
        0.26200 * 60.0,
        0.18968 * 60.0,
        0.13473 * 60.0,
        0.09388 * 60.0,
        0.06415 * 60.0,
        0.04298 * 60.0,
        0.02822 * 60.0,
        0.01815 * 60.0,
        0.01144 * 60.0,
        0.00706 * 60.0
    ];

    pub fn calculate_gravity(level: usize) -> f32 {
        GRAVITY_RATES_IN_SECONDS[std::cmp::min(level, GRAVITY_RATES_IN_SECONDS.len())]
    }
}