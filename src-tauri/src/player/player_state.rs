use crate::game::court::{self, CourtArea};
use crate::player::player_attributes;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub action: PlayerAction,
    pub has_ball: bool,
    pub current_area: court::CourtArea,
}

impl PlayerState {
    pub fn new(is_offense: bool, has_ball: bool, area: Option<court::CourtArea>) -> PlayerState {
        match area {
            Some(value) => {
                if is_offense {
                    PlayerState {
                        action: PlayerAction::Idle,
                        has_ball,
                        current_area: value,
                    }
                } else {
                    PlayerState {
                        action: PlayerAction::Idle,
                        has_ball,
                        current_area: value,
                    }
                }
            }
            None => {
                if is_offense {
                    PlayerState {
                        action: PlayerAction::Idle,
                        has_ball,
                        current_area: court::CourtArea::Center,
                    }
                } else {
                    PlayerState {
                        action: PlayerAction::Idle,
                        has_ball,
                        current_area: court::CourtArea::Center,
                    }
                }
            }
        }
    }
    pub fn generate_offensive_player_action(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
    ) {
        let actions;
        if self.has_ball {
            println!(
                "Shot Chance: {}, Area: {:?}",
                self.calculate_shot_chance(attributes),
                self.current_area
            );
            if self.calculate_shot_chance(attributes) < 0.4 {
                actions = vec![PlayerAction::Pass, PlayerAction::Drive]
            } else {
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
                        actions = vec![PlayerAction::Layup, PlayerAction::Dunk];
                    } else {
                        actions = vec![PlayerAction::Shoot];
                    }
                } else {
                    if self.action == PlayerAction::Drive {
                        actions = vec![PlayerAction::ShootOfDribble];
                    } else {
                        actions = vec![PlayerAction::Shoot, PlayerAction::ShootOfDribble];
                    }
                }
            }
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
        self.action = actions[index];
    }
    //Function that returns Some(2, 3) or None
    pub fn is_shot(&self) -> (bool, u8) {
        let shot_actions = [
            PlayerAction::Shoot,
            PlayerAction::ShootOfDribble,
            PlayerAction::Layup,
            PlayerAction::Dunk,
        ];
        let two_points = [
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::FreeThrowLine,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeBaselineLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
            CourtArea::RestrictedAreaLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeWingLeft,
            CourtArea::ShortCornerRight,
            CourtArea::ShortCornerLeft,
            CourtArea::MidrangeCenter,
        ];
        let three_points = [
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineCenter,
            CourtArea::Backcourt,
            CourtArea::Center,
        ];
        match self.action {
            action if shot_actions.contains(&action) => {
                if two_points.contains(&self.current_area) {
                    return (true, 2);
                } else if three_points.contains(&self.current_area) {
                    return (true, 3);
                } else {
                    return (false, 0);
                }
            }
            _ => return (false, 0),
        }
    }
    pub fn generate_defensive_player_action(&mut self) {
        let actions = vec![
            PlayerAction::Block,
            PlayerAction::DefendTight,
            PlayerAction::Defend,
            PlayerAction::DefendLoose,
            PlayerAction::Steal,
            PlayerAction::Foul,
        ];
        let index = rand::random::<usize>() % actions.len();
        self.action = actions[index]
    }
    pub fn generate_player_next_area(&mut self) {
        let (is_shot, _) = self.is_shot();
        if !is_shot {
            let available_areas = court::can_move_to(self.current_area);
            let index = rand::random::<usize>() % available_areas.len();
            self.current_area = available_areas.into_iter().nth(index).unwrap()
        }
    }

    pub fn calculate_shot_chance(&self, attributes: &player_attributes::PlayerAttributes) -> f32 {
        let area_shot_chance = self.current_area.shot_chance();
        let attributes_shot_chance = attributes.shot_chance(self.current_area);
        let shot_chance = (area_shot_chance * attributes_shot_chance) / 100.0;
        return shot_chance;
    }

    pub fn generate_next_player_state(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
        is_offense: bool,
        has_ball: bool,
    ) {
        self.has_ball = has_ball;

        if is_offense {
            self.generate_offensive_player_action(attributes);
        } else {
            self.generate_defensive_player_action();
        }
        self.generate_player_next_area();
    }
}
