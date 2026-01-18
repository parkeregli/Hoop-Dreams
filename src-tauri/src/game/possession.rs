use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Possession {
    Home,
    Away,
}

impl fmt::Display for Possession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Possession::Home => write!(f, "Home"),
            Possession::Away => write!(f, "Away"),
        }
    }
}

impl Possession {
    /// Returns the team index (Home=0, Away=1).
    pub fn team_index(&self) -> usize {
        match self {
            Possession::Home => 0,
            Possession::Away => 1,
        }
    }

    /// Returns the opposite possession.
    pub fn opposite(&self) -> Possession {
        match self {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        }
    }
}

/// Represents which team and player has the ball.
pub type BallPossession = Option<(Possession, usize)>;

/// Helper to get the team index from a possession state.
pub fn get_team_index(possession: &BallPossession) -> Option<usize> {
    possession.map(|(p, _)| p.team_index())
}

/// Helper to get the player index from a possession state.
pub fn get_player_index(possession: &BallPossession) -> Option<usize> {
    possession.map(|(_, idx)| idx)
}

/// Helper to switch possession to the other team with a given player index.
pub fn switch_possession(current: &BallPossession, new_player_index: usize) -> BallPossession {
    current.map(|(p, _)| (p.opposite(), new_player_index))
}
