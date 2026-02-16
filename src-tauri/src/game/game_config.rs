use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Game timing constants
pub const SHOT_CLOCK_SECS: u64 = 24;
pub const PERIOD_DURATION_SECS: u64 = 720; // 12 minutes
pub const OVERTIME_DURATION_SECS: u64 = 300; // 5 minutes
pub const BUZZER_BEATER_THRESHOLD_MS: u64 = 500;

/// Team and player counts
pub const PLAYERS_PER_TEAM: usize = 5;
pub const NUM_PERIODS: u8 = 4;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JumpTarget {
    Q2Start,
    Q3Start,
    Q4Start,
    GameEnd,
}

impl JumpTarget {
    pub fn target_period(&self) -> u8 {
        match self {
            JumpTarget::Q2Start => 2,
            JumpTarget::Q3Start => 3,
            JumpTarget::Q4Start => 4,
            JumpTarget::GameEnd => NUM_PERIODS,
        }
    }

    pub fn target_time(&self) -> Duration {
        match self {
            JumpTarget::Q2Start | JumpTarget::Q3Start | JumpTarget::Q4Start => period_duration(),
            JumpTarget::GameEnd => Duration::ZERO,
        }
    }

    pub fn is_game_over(&self) -> bool {
        *self == JumpTarget::GameEnd
    }
}

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
