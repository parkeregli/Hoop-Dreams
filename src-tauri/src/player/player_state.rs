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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerState {
    pub action: PlayerAction,
    pub current_area: court::CourtArea,
}

impl PlayerState {
    pub fn new(area: Option<court::CourtArea>) -> PlayerState {
        match area {
            Some(value) => PlayerState {
                action: PlayerAction::Idle,
                current_area: value,
            },
            None => PlayerState {
                action: PlayerAction::Idle,
                current_area: court::CourtArea::Center,
            },
        }
    }
    pub fn generate_offensive_player_action(
        &mut self,
        attributes: &player_attributes::PlayerAttributes,
        has_ball: bool,
    ) {
        let actions;
        if has_ball {
            let shot_chance = self.calculate_shot_chance(attributes);
            if shot_chance < 0.3 {
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
                        actions = vec![PlayerAction::Shoot];
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
    pub fn is_shot(&self) -> Option<u8> {
        let shot_actions = [
            PlayerAction::Shoot,
            PlayerAction::ShootOfDribble,
            PlayerAction::Layup,
            PlayerAction::Dunk,
        ];
        let points = self.current_area.points();
        match self.action {
            action if shot_actions.contains(&action) => Some(points),
            _ => return None,
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
    pub fn generate_defense_player_next_area(&mut self, opp_area: CourtArea) {
        if is_between_basket(self.current_area, opp_area) {
            self.current_area = go_towards(self.current_area, opp_area);
        } else {
            let goal_area = court::defend_towards(self.current_area, opp_area);
            self.current_area = go_towards(self.current_area, goal_area)
        }
    }
    pub fn generate_offensive_player_next_area(&mut self) {
        let available_areas = court::can_move_to(self.current_area);
        let index = rand::random::<usize>() % available_areas.len();
        self.current_area = available_areas.into_iter().nth(index).unwrap()
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
        is_offense: (bool, bool),
        is_defense: (bool, Option<&PlayerState>),
    ) -> Result<(), String> {
        if is_offense.0 == is_defense.0 {
            return Err("Both offense and defense are the same".to_string());
        }
        if is_offense.0 {
            self.generate_offensive_player_action(attributes, is_offense.1);
            self.generate_offensive_player_next_area();
            return Ok(());
        } else if is_defense.0 {
            match is_defense.1 {
                Some(state) => {
                    self.generate_defense_player_next_area(state.current_area);
                }
                None => self.generate_defensive_player_action(),
            }
            self.generate_defensive_player_action();
            return Ok(());
        }

        Err("Neither offense nor defense".to_string())
    }
}
