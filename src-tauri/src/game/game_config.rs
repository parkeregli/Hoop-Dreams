use std::time::Duration;

/// Game timing constants
pub const SHOT_CLOCK_SECS: u64 = 24;
pub const PERIOD_DURATION_SECS: u64 = 720; // 12 minutes
pub const OVERTIME_DURATION_SECS: u64 = 300; // 5 minutes
pub const BUZZER_BEATER_THRESHOLD_MS: u64 = 500;

/// Team and player counts
pub const PLAYERS_PER_TEAM: usize = 5;
pub const NUM_PERIODS: u8 = 4;

/// Helper functions
pub fn shot_clock_duration() -> Duration {
    Duration::from_secs(SHOT_CLOCK_SECS)
}

pub fn period_duration() -> Duration {
    Duration::from_secs(PERIOD_DURATION_SECS)
}

pub fn overtime_duration() -> Duration {
    Duration::from_secs(OVERTIME_DURATION_SECS)
}

pub fn buzzer_beater_threshold() -> Duration {
    Duration::from_millis(BUZZER_BEATER_THRESHOLD_MS)
}
