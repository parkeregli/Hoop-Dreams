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
    Cut,
    Screen,
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
}

impl PlayerState {
    pub fn new(is_offense: bool, has_ball: bool) -> PlayerState {
        if is_offense {
            PlayerState {
                action: PlayerState::generate_offensive_player_action(has_ball),
                has_ball,
            }
        } else {
            PlayerState {
                action: PlayerState::generate_deffensive_player_action(has_ball),
                has_ball,
            }
        }
    }
    pub fn generate_offensive_player_action(has_ball: bool) -> PlayerAction {
        let mut actions = vec![];
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
                PlayerAction::Screen,
            ];
        }
        let index = rand::random::<usize>() % actions.len();
        actions[index]
    }

    pub fn generate_deffensive_player_action(is_defened_ball: bool) -> PlayerAction {
        let mut actions = vec![];
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
