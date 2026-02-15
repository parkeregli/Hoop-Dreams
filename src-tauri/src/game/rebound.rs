use crate::game::court::CourtArea;
use crate::game::game_config::PLAYERS_PER_TEAM;
use crate::game::possession::BallPossession;
use crate::game::Game;
use crate::player::player_state::PlayerState;
use crate::player::Player;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReboundWinner {
    pub team_index: usize,
    pub player_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReboundZone {
    Restricted,
    InsideArc,
    Midrange,
    ThreePoint,
    Long,
}

fn get_rebound_zone_type(area: CourtArea) -> ReboundZone {
    match area {
        CourtArea::Basket
        | CourtArea::RestrictedAreaLeft
        | CourtArea::RestrictedAreaMiddle
        | CourtArea::RestrictedAreaRight => ReboundZone::Restricted,

        CourtArea::LowPostLeft
        | CourtArea::LowPostRight
        | CourtArea::ShortCornerLeft
        | CourtArea::ShortCornerRight
        | CourtArea::ElbowLeft
        | CourtArea::ElbowRight
        | CourtArea::FreeThrowLine => ReboundZone::InsideArc,

        CourtArea::MidrangeBaselineLeft
        | CourtArea::MidrangeBaselineRight
        | CourtArea::MidrangeWingLeft
        | CourtArea::MidrangeWingRight
        | CourtArea::MidrangeCenter => ReboundZone::Midrange,

        CourtArea::ThreePointLineCornerLeft
        | CourtArea::ThreePointLineCornerRight
        | CourtArea::ThreePointLineWingLeft
        | CourtArea::ThreePointLineWingRight
        | CourtArea::ThreePointLineCenter => ReboundZone::ThreePoint,

        CourtArea::Center
        | CourtArea::Backcourt
        | CourtArea::SidelineLeft
        | CourtArea::SidelineRight
        | CourtArea::BaselineLeft
        | CourtArea::BaselineRight
        | CourtArea::OutOfBounds => ReboundZone::Long,
    }
}

fn get_zones_for_rebound_zone(zone: ReboundZone) -> Vec<CourtArea> {
    match zone {
        ReboundZone::Restricted => vec![
            CourtArea::Basket,
            CourtArea::RestrictedAreaLeft,
            CourtArea::RestrictedAreaMiddle,
            CourtArea::RestrictedAreaRight,
        ],
        ReboundZone::InsideArc => vec![
            CourtArea::LowPostLeft,
            CourtArea::LowPostRight,
            CourtArea::ShortCornerLeft,
            CourtArea::ShortCornerRight,
            CourtArea::ElbowLeft,
            CourtArea::ElbowRight,
            CourtArea::FreeThrowLine,
        ],
        ReboundZone::Midrange => vec![
            CourtArea::MidrangeBaselineLeft,
            CourtArea::MidrangeBaselineRight,
            CourtArea::MidrangeWingLeft,
            CourtArea::MidrangeWingRight,
            CourtArea::MidrangeCenter,
        ],
        ReboundZone::ThreePoint => vec![
            CourtArea::ThreePointLineCornerLeft,
            CourtArea::ThreePointLineCornerRight,
            CourtArea::ThreePointLineWingLeft,
            CourtArea::ThreePointLineWingRight,
            CourtArea::ThreePointLineCenter,
        ],
        ReboundZone::Long => vec![
            CourtArea::Center,
            CourtArea::SidelineLeft,
            CourtArea::SidelineRight,
            CourtArea::BaselineLeft,
            CourtArea::BaselineRight,
        ],
    }
}

fn pick_side_based_on_shot(_shot_zone: ReboundZone, shot_area: CourtArea) -> (bool, bool, bool) {
    let mut left = true;
    let mut right = true;
    let mut center = true;

    match shot_area {
        CourtArea::RestrictedAreaLeft
        | CourtArea::LowPostLeft
        | CourtArea::ShortCornerLeft
        | CourtArea::ElbowLeft
        | CourtArea::MidrangeBaselineLeft
        | CourtArea::MidrangeWingLeft
        | CourtArea::ThreePointLineCornerLeft
        | CourtArea::ThreePointLineWingLeft => {
            left = true;
            right = false;
            center = false;
        }
        CourtArea::RestrictedAreaRight
        | CourtArea::LowPostRight
        | CourtArea::ShortCornerRight
        | CourtArea::ElbowRight
        | CourtArea::MidrangeBaselineRight
        | CourtArea::MidrangeWingRight
        | CourtArea::ThreePointLineCornerRight
        | CourtArea::ThreePointLineWingRight => {
            left = false;
            right = true;
            center = false;
        }
        CourtArea::RestrictedAreaMiddle
        | CourtArea::FreeThrowLine
        | CourtArea::MidrangeCenter
        | CourtArea::ThreePointLineCenter => {
            left = false;
            right = false;
            center = true;
        }
        _ => {}
    }

    (left, right, center)
}

pub fn pick_rebound_zone(shot_area: CourtArea) -> CourtArea {
    let mut rng = thread_rng();
    let shot_zone = get_rebound_zone_type(shot_area);

    let roll: f32 = rng.gen_range(0.0..1.0);

    let (allow_left, allow_right, allow_center) = pick_side_based_on_shot(shot_zone, shot_area);

    let (zone_weights, _zone_areas): (Vec<(&str, f32)>, Vec<CourtArea>) =
        if shot_zone == ReboundZone::Restricted || shot_zone == ReboundZone::InsideArc {
            (
                vec![
                    ("Restricted", 0.50),
                    ("InsideArc", 0.30),
                    ("Midrange", 0.12),
                    ("ThreePoint", 0.06),
                    ("Long", 0.02),
                ],
                get_zones_for_rebound_zone(ReboundZone::Restricted)
                    .into_iter()
                    .chain(get_zones_for_rebound_zone(ReboundZone::InsideArc))
                    .chain(get_zones_for_rebound_zone(ReboundZone::Midrange))
                    .chain(get_zones_for_rebound_zone(ReboundZone::ThreePoint))
                    .chain(get_zones_for_rebound_zone(ReboundZone::Long))
                    .collect(),
            )
        } else if shot_zone == ReboundZone::Midrange {
            (
                vec![
                    ("InsideArc", 0.40),
                    ("Midrange", 0.35),
                    ("ThreePoint", 0.15),
                    ("Long", 0.10),
                ],
                get_zones_for_rebound_zone(ReboundZone::InsideArc)
                    .into_iter()
                    .chain(get_zones_for_rebound_zone(ReboundZone::Midrange))
                    .chain(get_zones_for_rebound_zone(ReboundZone::ThreePoint))
                    .chain(get_zones_for_rebound_zone(ReboundZone::Long))
                    .collect(),
            )
        } else {
            (
                vec![("ThreePoint", 0.45), ("Midrange", 0.35), ("Long", 0.20)],
                get_zones_for_rebound_zone(ReboundZone::ThreePoint)
                    .into_iter()
                    .chain(get_zones_for_rebound_zone(ReboundZone::Midrange))
                    .chain(get_zones_for_rebound_zone(ReboundZone::Long))
                    .collect(),
            )
        };

    let mut cumulative = 0.0;
    let mut selected_zone_type: Option<ReboundZone> = None;

    for (zone_name, weight) in zone_weights {
        cumulative += weight;
        if roll < cumulative {
            selected_zone_type = Some(match zone_name {
                "Restricted" => ReboundZone::Restricted,
                "InsideArc" => ReboundZone::InsideArc,
                "Midrange" => ReboundZone::Midrange,
                "ThreePoint" => ReboundZone::ThreePoint,
                "Long" => ReboundZone::Long,
                _ => ReboundZone::Restricted,
            });
            break;
        }
    }

    let selected_zone_type = selected_zone_type.unwrap_or(ReboundZone::Restricted);

    let mut candidate_areas = get_zones_for_rebound_zone(selected_zone_type);

    candidate_areas.retain(|area| match area {
        CourtArea::RestrictedAreaLeft
        | CourtArea::LowPostLeft
        | CourtArea::ShortCornerLeft
        | CourtArea::ElbowLeft
        | CourtArea::MidrangeBaselineLeft
        | CourtArea::MidrangeWingLeft
        | CourtArea::ThreePointLineCornerLeft
        | CourtArea::ThreePointLineWingLeft
        | CourtArea::SidelineLeft
        | CourtArea::BaselineLeft => allow_left,

        CourtArea::RestrictedAreaRight
        | CourtArea::LowPostRight
        | CourtArea::ShortCornerRight
        | CourtArea::ElbowRight
        | CourtArea::MidrangeBaselineRight
        | CourtArea::MidrangeWingRight
        | CourtArea::ThreePointLineCornerRight
        | CourtArea::ThreePointLineWingRight
        | CourtArea::SidelineRight
        | CourtArea::BaselineRight => allow_right,

        CourtArea::Basket
        | CourtArea::RestrictedAreaMiddle
        | CourtArea::FreeThrowLine
        | CourtArea::MidrangeCenter
        | CourtArea::ThreePointLineCenter
        | CourtArea::Center => allow_center,

        _ => true,
    });

    if candidate_areas.is_empty() {
        candidate_areas = get_zones_for_rebound_zone(ReboundZone::Restricted);
    }

    let area_roll = rng.gen_range(0..candidate_areas.len());
    candidate_areas[area_roll]
}

pub struct ReboundContender {
    pub team_index: usize,
    pub player_index: usize,
    pub player: Player,
    pub player_state: PlayerState,
    pub is_offensive: bool,
}

pub fn get_players_near_rebound_zone(game: &Game, zone: CourtArea) -> Vec<ReboundContender> {
    let mut contenders: Vec<ReboundContender> = Vec::new();

    for team_idx in 0..2 {
        for player_idx in 0..PLAYERS_PER_TEAM {
            let player = &game.state.team_state[team_idx].active_players[player_idx].0;
            let player_state = &game.state.team_state[team_idx].active_players[player_idx].1;

            let is_same_zone = player_state.current_area == zone;
            let is_adjacent = player_state.current_area.is_adjacent_to(zone);

            if is_same_zone || is_adjacent {
                let Some(possession) = &game.state.possession else {
                    continue;
                };
                let offensive_team = possession.0.team_index();
                let is_offensive = team_idx == offensive_team;

                contenders.push(ReboundContender {
                    team_index: team_idx,
                    player_index: player_idx,
                    player: player.clone(),
                    player_state: player_state.clone(),
                    is_offensive,
                });
            }
        }
    }

    if contenders.is_empty() {
        for team_idx in 0..2 {
            for player_idx in 0..PLAYERS_PER_TEAM {
                let player = &game.state.team_state[team_idx].active_players[player_idx].0;
                let player_state = &game.state.team_state[team_idx].active_players[player_idx].1;

                let Some(possession) = &game.state.possession else {
                    continue;
                };
                let offensive_team = possession.0.team_index();
                let is_offensive = team_idx == offensive_team;

                contenders.push(ReboundContender {
                    team_index: team_idx,
                    player_index: player_idx,
                    player: player.clone(),
                    player_state: player_state.clone(),
                    is_offensive,
                });
            }
        }
    }

    contenders
}

pub fn resolve_rebound(
    contenders: &[ReboundContender],
    rebound_zone: CourtArea,
) -> Option<ReboundWinner> {
    if contenders.is_empty() {
        return None;
    }

    let mut rng = thread_rng();

    let mut best_score: f32 = -1.0;
    let mut winner: Option<ReboundWinner> = None;

    for contender in contenders {
        let attributes = contender.player.attributes();
        let rebounding_stat = if contender.is_offensive {
            attributes.off_rebound as f32
        } else {
            attributes.def_rebound as f32
        };

        let proximity_bonus = if contender.player_state.current_area == rebound_zone {
            20.0
        } else if contender
            .player_state
            .current_area
            .is_adjacent_to(rebound_zone)
        {
            10.0
        } else {
            0.0
        };

        let height = contender.player.get_height() as f32;
        let height_bonus = (height - 60.0).max(0.0) * 0.5;

        let random_factor: f32 = rng.gen_range(-15.0..15.0);

        let total_score = rebounding_stat + proximity_bonus + height_bonus + random_factor;

        if total_score > best_score {
            best_score = total_score;
            winner = Some(ReboundWinner {
                team_index: contender.team_index,
                player_index: contender.player_index,
            });
        }
    }

    winner
}

pub fn is_offensive_rebound(winner: &ReboundWinner, possession: &BallPossession) -> bool {
    if let Some((pos, _)) = possession {
        winner.team_index == pos.team_index()
    } else {
        false
    }
}
