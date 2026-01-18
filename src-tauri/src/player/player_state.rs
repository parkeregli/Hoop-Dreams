use crate::game::court::{self, go_towards, is_between_basket, CourtArea};
use crate::player::player_attributes;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlayerAction {
    Pass,
    Drive,
    Rebound,
    Layup,
    Dunk,
    ShootOfDribble,
    Shoot,
    SpotUp,
    Cut,
    BallScreen,
    OffBallScreen,
    Contest,
    Block,
    DefendTight,
    Defend,
    DefendLoose,
    Steal,
    Foul,
    Idle,
}

impl fmt::Display for PlayerAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a player's role in the current play.
/// This replaces the confusing tuple parameters with clear semantic meaning.
#[derive(Debug, Clone, Copy)]
pub enum PlayerRole<'a> {
    /// Player is on offense and has the ball.
    BallHandler,
    /// Player is on offense but does not have the ball.
    OffBallOffense,
    /// Player is on defense, tracking an opponent.
    Defense {
        /// The opponent player state to track (for positioning).
        tracking: Option<&'a PlayerState>,
    },
}

impl PlayerRole<'_> {
    /// Returns true if the player is on offense.
    pub fn is_offense(&self) -> bool {
        matches!(self, PlayerRole::BallHandler | PlayerRole::OffBallOffense)
    }

    /// Returns true if the player has the ball.
    pub fn has_ball(&self) -> bool {
        matches!(self, PlayerRole::BallHandler)
    }

    /// Returns true if the player is on defense.
    pub fn is_defense(&self) -> bool {
        matches!(self, PlayerRole::Defense { .. })
    }

    /// Converts to the legacy tuple format for backward compatibility.
    /// Returns (is_offense, has_ball), (is_defense, tracking)
    #[allow(dead_code)]
    pub fn to_legacy(&self) -> ((bool, bool), (bool, Option<&PlayerState>)) {
        match self {
            PlayerRole::BallHandler => ((true, true), (false, None)),
            PlayerRole::OffBallOffense => ((true, false), (false, None)),
            PlayerRole::Defense { tracking } => ((false, false), (true, *tracking)),
        }
    }

    /// Creates a PlayerRole from the legacy tuple format.
    pub fn from_legacy(
        is_offense: (bool, bool),
        is_defense: (bool, Option<&PlayerState>),
    ) -> Result<PlayerRole<'_>, &'static str> {
        if is_offense.0 && is_defense.0 {
            return Err("Cannot be both offense and defense");
        }
        if !is_offense.0 && !is_defense.0 {
            return Err("Must be either offense or defense");
        }

        if is_offense.0 {
            if is_offense.1 {
                Ok(PlayerRole::BallHandler)
            } else {
                Ok(PlayerRole::OffBallOffense)
            }
        } else {
            Ok(PlayerRole::Defense {
                tracking: is_defense.1,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub action: PlayerAction,
    pub current_area: court::CourtArea,
}

impl PlayerState {
    pub fn new(area: Option<court::CourtArea>) -> PlayerState {
        PlayerState {
            action: PlayerAction::Idle,
            current_area: area.unwrap_or(court::CourtArea::Center),
        }
    }

    pub fn generate_offensive_player_action(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
        has_ball: bool,
    ) {
        let actions = if has_ball {
            self.get_ball_handler_actions(attributes)
        } else {
            self.get_off_ball_actions()
        };

        let index = rand::random::<usize>() % actions.len();
        self.action = actions[index];
    }

    fn get_ball_handler_actions(
        &self,
        attributes: &player_attributes::PlayerAttributes,
    ) -> Vec<PlayerAction> {
        let shot_chance = self.calculate_shot_chance(attributes);

        if shot_chance < 0.3 {
            return vec![PlayerAction::Pass, PlayerAction::Drive];
        }

        let inside_shot_areas = [
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerLeft,
            CourtArea::ShortCornerRight,
        ];

        if inside_shot_areas.contains(&self.current_area) {
            if self.action == PlayerAction::Drive {
                vec![PlayerAction::Layup, PlayerAction::Dunk]
            } else {
                vec![PlayerAction::Shoot]
            }
        } else if self.action == PlayerAction::Drive {
            vec![PlayerAction::ShootOfDribble]
        } else {
            vec![PlayerAction::Shoot]
        }
    }

    fn get_off_ball_actions(&self) -> Vec<PlayerAction> {
        vec![
            PlayerAction::Rebound,
            PlayerAction::Cut,
            PlayerAction::BallScreen,
            PlayerAction::OffBallScreen,
            PlayerAction::SpotUp,
        ]
    }

    /// Returns Some(points) if the current action is a shot, None otherwise.
    pub fn is_shot(&self) -> Option<u8> {
        const SHOT_ACTIONS: [PlayerAction; 4] = [
            PlayerAction::Shoot,
            PlayerAction::ShootOfDribble,
            PlayerAction::Layup,
            PlayerAction::Dunk,
        ];

        if SHOT_ACTIONS.contains(&self.action) {
            Some(self.current_area.points())
        } else {
            None
        }
    }

    pub fn generate_defensive_player_action(&mut self) {
        const DEFENSIVE_ACTIONS: [PlayerAction; 6] = [
            PlayerAction::Block,
            PlayerAction::DefendTight,
            PlayerAction::Defend,
            PlayerAction::DefendLoose,
            PlayerAction::Steal,
            PlayerAction::Foul,
        ];

        let index = rand::random::<usize>() % DEFENSIVE_ACTIONS.len();
        self.action = DEFENSIVE_ACTIONS[index];
    }

    pub fn generate_defense_player_next_area(&mut self, opp_area: CourtArea) {
        if is_between_basket(self.current_area, opp_area) {
            self.current_area = go_towards(self.current_area, opp_area);
        } else {
            let goal_area = court::defend_towards(self.current_area, opp_area);
            self.current_area = go_towards(self.current_area, goal_area);
        }
    }

    pub fn generate_offensive_player_next_area(&mut self) {
        let available_areas = court::can_move_to(self.current_area);
        let index = rand::random::<usize>() % available_areas.len();
        self.current_area = available_areas.into_iter().nth(index).unwrap();
    }

    pub fn calculate_shot_chance(&self, attributes: &player_attributes::PlayerAttributes) -> f32 {
        let area_shot_chance = self.current_area.shot_chance();
        let attributes_shot_chance = attributes.shot_chance(self.current_area);
        (area_shot_chance * attributes_shot_chance) / 100.0
    }

    /// Updates the player state based on their role in the current play.
    pub fn update_for_role(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
        role: PlayerRole<'_>,
    ) -> Result<(), String> {
        match role {
            PlayerRole::BallHandler => {
                self.generate_offensive_player_action(attributes, true);
                self.generate_offensive_player_next_area();
            }
            PlayerRole::OffBallOffense => {
                self.generate_offensive_player_action(attributes, false);
                self.generate_offensive_player_next_area();
            }
            PlayerRole::Defense { tracking } => {
                if let Some(opponent_state) = tracking {
                    self.generate_defense_player_next_area(opponent_state.current_area);
                }
                self.generate_defensive_player_action();
            }
        }
        Ok(())
    }

    /// Legacy method - prefer using `update_for_role` with `PlayerRole` enum.
    pub fn generate_next_player_state(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
        is_offense: (bool, bool),
        is_defense: (bool, Option<&PlayerState>),
    ) -> Result<(), String> {
        let role = PlayerRole::from_legacy(is_offense, is_defense)
            .map_err(|e| e.to_string())?;
        self.update_for_role(attributes, role)
    }
}
