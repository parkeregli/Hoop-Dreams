use crate::game::court::CourtArea;
use crate::player::Player;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerAction {
    Pass,
    Drive,
    Rebound,
    InsShot,
    MidShot,
    CornerThree,
    Three,
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
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub action: PlayerAction,
    pub has_ball: bool,
    pub current_area: CourtArea,
}

impl PlayerState {
    pub fn new(player: &Player, is_offense: bool, has_ball: bool) -> PlayerState {
        if is_offense {
            PlayerState {
                action: PlayerState::generate_offensive_player_action(player, has_ball),
                has_ball,
                current_area: CourtArea::Center,
            }
        } else {
            PlayerState {
                action: PlayerState::generate_deffensive_player_action(player, has_ball),
                has_ball,
                current_area: CourtArea::Center,
            }
        }
    }
    pub fn generate_offensive_player_action(player: &Player, has_ball: bool) -> PlayerAction {
        let actions;
        if has_ball {
            actions = vec![
                PlayerAction::Pass,
                PlayerAction::Drive,
                PlayerAction::InsShot,
                PlayerAction::MidShot,
                PlayerAction::CornerThree,
                PlayerAction::Three,
            ];
        } else {
            actions = vec![
                PlayerAction::Rebound,
                PlayerAction::Cut,
                PlayerAction::BallScreen,
                PlayerAction::OffBallScreen,
                PlayerAction::SpotUp,
            ];
        }
        let index = rand::random::<usize>() % actions.len();
        actions[index]
    }

    pub fn generate_deffensive_player_action(
        player: &Player,
        is_defened_ball: bool,
    ) -> PlayerAction {
        let mut actions;
        if is_defened_ball {
            actions = vec![
                PlayerAction::Block,
                PlayerAction::DefendTight,
                PlayerAction::Defend,
                PlayerAction::DefendLoose,
                PlayerAction::Steal,
            ];
        } else {
            actions = vec![
                PlayerAction::DefendTight,
                PlayerAction::Defend,
                PlayerAction::DefendLoose,
                PlayerAction::Rebound,
            ];
        }
        let index = rand::random::<usize>() % actions.len();
        actions[index]
    }
}
